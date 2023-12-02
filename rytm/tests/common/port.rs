use anyhow::Result;
use midir::{Ignore, MidiInput, MidiInputPort, MidiOutput, MidiOutputConnection, MidiOutputPort};

pub struct MidiIn {
    inner: MidiInput,
}
impl MidiIn {
    pub fn new(name: &str) -> Result<Self> {
        let mut inner = MidiInput::new(name)?;
        // Set here if necessary
        inner.ignore(Ignore::None);
        Ok(Self { inner })
    }

    pub fn ignore(&mut self, ignore: Ignore) {
        self.inner.ignore(ignore);
    }

    pub fn into_inner(self) -> MidiInput {
        self.inner
    }

    pub fn inner(&self) -> &MidiInput {
        &self.inner
    }

    pub fn find_input_port(&self, pattern: &str) -> Option<MidiInputPort> {
        self.inner
            .ports()
            .into_iter()
            .find(|port| self.inner.port_name(port).unwrap().contains(pattern))
    }

    pub fn list_all_to_stdout(&self) {
        println!("Available input ports:");
        for (i, p) in self.inner().ports().iter().enumerate() {
            println!("{}: {}", i, self.inner().port_name(p).unwrap());
        }
    }
}

pub struct MidiOut {
    inner: MidiOutput,
}
impl MidiOut {
    pub fn new(name: &str) -> Result<Self> {
        let inner = MidiOutput::new(name)?;
        Ok(Self { inner })
    }

    pub fn into_inner(self) -> MidiOutput {
        self.inner
    }

    pub fn inner(&self) -> &MidiOutput {
        &self.inner
    }

    pub fn find_output_port(&self, pattern: &str) -> Option<MidiOutputPort> {
        self.inner
            .ports()
            .into_iter()
            .find(|port| self.inner.port_name(port).unwrap().contains(pattern))
    }

    pub fn make_output_connection(
        self,
        output_port: &MidiOutputPort,
        index: u8,
    ) -> Result<MidiOutputConnection> {
        let name = self.inner.port_name(output_port)?;
        let connection = self
            .into_inner()
            // Duplicate names?
            .connect(output_port, &format!("{name}_{index}"))
            .map_err(|err| anyhow::anyhow!("Error connecting to midi output port: {}", err))?;
        Ok(connection)
    }

    pub fn list_all_to_stdout(&self) {
        println!("Available output ports:");
        for (i, p) in self.inner().ports().iter().enumerate() {
            println!("{}: {}", i, self.inner().port_name(p).unwrap());
        }
    }
}
