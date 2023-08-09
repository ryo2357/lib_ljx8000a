use chrono::{DateTime, Local};
use log::{info, warn};
use std::sync::{Arc, Mutex};

use crate::interface::ReceiveData;

use super::config::Ljx8000aConfig;
use super::interface::LjxIf;
use super::profile_writer::ProfileWriter;

pub struct LjxClient {
    config: Ljx8000aConfig,
    interface: LjxIf,
    data_receiver: Arc<Mutex<std::sync::mpsc::Receiver<ReceiveData>>>,
    profile_writer: Option<ProfileWriter>,
    state: Ljx8060State,
}
impl LjxClient {
    pub async fn create(config: Ljx8000aConfig) -> anyhow::Result<Self> {
        let (interface, data_receiver) = match LjxIf::create() {
            Ok(t) => t,
            Err(err) => panic!(
                "Error when ffi::LJX8IF_InitializeHighSpeedDataCommunication:{:?}",
                err
            ),
        };

        Ok(Self {
            config,
            interface,
            data_receiver: Arc::new(Mutex::new(data_receiver)),
            profile_writer: None,
            state: Ljx8060State::NoConnection,
        })
    }

    pub async fn open_ethernet(&mut self) -> anyhow::Result<()> {
        if self.state != Ljx8060State::NoConnection {
            warn!("not NoConnection: state = {:?}", self.state);
            anyhow::bail!("not NoConnection: state = {:?}", self.state)
        }

        match self
            .interface
            .open_ethernet(self.config.host, self.config.port)
        {
            Ok(_t) => {}
            Err(err) => anyhow::bail!("{:?}", err),
        }

        self.state = Ljx8060State::OpenedEthernet;
        Ok(())
    }

    pub async fn initialize_communication(&mut self) -> anyhow::Result<()> {
        if self.state != Ljx8060State::OpenedEthernet {
            warn!("not OpenedEthernet: state = {:?}", self.state);
            anyhow::bail!("not OpenedEthernet: state = {:?}", self.state)
        }
        match self
            .interface
            .initialize_communication(self.config.high_speed_port)
        {
            Ok(_) => {}
            Err(err) => anyhow::bail!("{:?}", err),
        }

        self.state = Ljx8060State::Initialized;
        Ok(())
    }

    pub async fn pre_start_communication(&mut self) -> anyhow::Result<()> {
        if self.state != Ljx8060State::Initialized {
            warn!("not Initialized: state = {:?}", self.state);
            anyhow::bail!("not Initialized: state = {:?}", self.state)
        }
        match self.interface.pre_start_communication() {
            Ok(_) => {}
            Err(err) => anyhow::bail!("{:?}", err),
        }

        // プレスタート時に作成
        let date = get_time_string();
        let save_path = self.config.save_dir.clone() + "/raw_profile" + &date + ".hex";

        info!("make profile : {:?}", save_path);

        let rx = Arc::clone(&self.data_receiver);
        let profile_writer = ProfileWriter::new(
            rx,
            save_path,
            self.config.profile_data_num,
            self.config.fetch_brightness_data,
        )?;

        self.state = Ljx8060State::PreStarted;
        self.profile_writer = Some(profile_writer);

        Ok(())
    }

    pub async fn start_communication(&mut self) -> anyhow::Result<()> {
        if self.state != Ljx8060State::PreStarted {
            warn!("not PreStarted: state = {:?}", self.state);
            anyhow::bail!("not PreStarted: state = {:?}", self.state)
        }
        match self.interface.start_communication() {
            Ok(_) => {}
            Err(err) => anyhow::bail!("{:?}", err),
        }

        self.state = Ljx8060State::Measuring;
        Ok(())
    }

    pub async fn stop_communication(&mut self) -> anyhow::Result<()> {
        if self.state != Ljx8060State::Measuring {
            warn!("not Measuring: state = {:?}", self.state);
            anyhow::bail!("not Measuring: state = {:?}", self.state)
        }
        match self.interface.stop_communication() {
            Ok(_) => {}
            Err(err) => anyhow::bail!("{:?}", err),
        }
        self.profile_writer = None;
        self.state = Ljx8060State::PreStarted;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum Ljx8060State {
    NoConnection,
    OpenedEthernet,
    Initialized,
    PreStarted,
    Measuring,
}

fn get_time_string() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d_%H%M%S").to_string()
}
