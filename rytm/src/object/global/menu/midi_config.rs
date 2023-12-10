use super::super::types::{
    MidiParameterOutput, MidiPortFunction, MidiPortsOutputChannel, MidiTransportLayer,
    ParameterDestination,
};
use crate::{
    error::{ConversionError, ParameterError, RytmError},
    object::global::types::MidiChannel,
};
use derivative::Derivative;
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_global_t;

/// Represents the `Midi Config` menu.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MidiConfig {
    sync: Sync,
    port_config: PortConfig,
    channels: Channels,
}

impl TryFrom<&ar_global_t> for MidiConfig {
    type Error = ConversionError;
    fn try_from(raw_global: &ar_global_t) -> Result<Self, Self::Error> {
        Ok(Self {
            sync: raw_global.try_into()?,
            port_config: raw_global.try_into()?,
            channels: raw_global.into(),
        })
    }
}

impl MidiConfig {
    pub(crate) fn apply_to_raw_global(&self, raw_global: &mut ar_global_t) {
        self.sync.apply_to_raw_global(raw_global);
        self.port_config.apply_to_raw_global(raw_global);
        self.channels.apply_to_raw_global(raw_global);
    }

    /// Returns the `Sync` menu.
    pub fn sync(&self) -> &Sync {
        &self.sync
    }

    /// Returns the `Port Config` menu.
    pub fn port_config(&self) -> &PortConfig {
        &self.port_config
    }

    /// Returns the `Channels` menu.
    pub fn channels(&self) -> &Channels {
        &self.channels
    }

    /// Returns the `Sync` menu as mutable.
    pub fn sync_mut(&mut self) -> &mut Sync {
        &mut self.sync
    }

    /// Returns the `Port Config` menu as mutable.
    pub fn port_config_mut(&mut self) -> &mut PortConfig {
        &mut self.port_config
    }

    /// Returns the `Channels` menu as mutable.
    pub fn channels_mut(&mut self) -> &mut Channels {
        &mut self.channels
    }
}

/// Represents the `Sync` menu in `Midi Config` menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sync {
    clock_receive: bool,
    clock_send: bool,
    transport_receive: bool,
    transport_send: bool,
    program_change_receive: bool,
    program_change_send: bool,
}

impl Default for Sync {
    fn default() -> Self {
        Self {
            clock_receive: true,
            clock_send: false,
            transport_receive: true,
            transport_send: false,
            program_change_receive: false,
            program_change_send: false,
        }
    }
}

impl TryFrom<&ar_global_t> for Sync {
    type Error = ConversionError;
    fn try_from(raw_global: &ar_global_t) -> Result<Self, Self::Error> {
        Ok(Self {
            clock_receive: raw_global.clock_receive != 0,
            clock_send: raw_global.clock_send != 0,
            transport_receive: raw_global.transport_receive != 0,
            transport_send: raw_global.transport_send != 0,
            program_change_receive: raw_global.program_change_receive != 0,
            program_change_send: raw_global.program_change_send != 0,
        })
    }
}

impl Sync {
    pub(crate) fn apply_to_raw_global(&self, raw_global: &mut ar_global_t) {
        raw_global.clock_receive = self.clock_receive as u8;
        raw_global.clock_send = self.clock_send as u8;
        raw_global.transport_receive = self.transport_receive as u8;
        raw_global.transport_send = self.transport_send as u8;
        raw_global.program_change_receive = self.program_change_receive as u8;
        raw_global.program_change_send = self.program_change_send as u8;
    }

    /// Turns clock receive on or off.
    pub fn set_clock_receive(&mut self, clock_receive: bool) {
        self.clock_receive = clock_receive;
    }

    /// Turns clock send on or off.
    pub fn set_clock_send(&mut self, clock_send: bool) {
        self.clock_send = clock_send;
    }

    /// Turns transport receive on or off.
    pub fn set_transport_receive(&mut self, transport_receive: bool) {
        self.transport_receive = transport_receive;
    }

    /// Turns transport send on or off.
    pub fn set_transport_send(&mut self, transport_send: bool) {
        self.transport_send = transport_send;
    }

    /// Turns program change receive on or off.
    pub fn set_program_change_receive(&mut self, program_change_receive: bool) {
        self.program_change_receive = program_change_receive;
    }

    /// Turns program change send on or off.
    pub fn set_program_change_send(&mut self, program_change_send: bool) {
        self.program_change_send = program_change_send;
    }

    /// Returns `true` if clock receive is on.
    pub fn clock_receive(&self) -> bool {
        self.clock_receive
    }

    /// Returns `true` if clock send is on.
    pub fn clock_send(&self) -> bool {
        self.clock_send
    }

    /// Returns `true` if transport receive is on.
    pub fn transport_receive(&self) -> bool {
        self.transport_receive
    }

    /// Returns `true` if transport send is on.
    pub fn transport_send(&self) -> bool {
        self.transport_send
    }

    /// Returns `true` if program change receive is on.
    pub fn program_change_receive(&self) -> bool {
        self.program_change_receive
    }

    /// Returns `true` if program change send is on.
    pub fn program_change_send(&self) -> bool {
        self.program_change_send
    }
}

/// Represents the `Port Config` menu in `Midi Config` menu.
#[derive(Derivative, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derivative(Debug)]
pub struct PortConfig {
    out_port_func: MidiPortFunction,
    thru_port_func: MidiPortFunction,
    input_from: MidiTransportLayer,
    output_to: MidiTransportLayer,
    param_output: MidiParameterOutput,
    receive_notes: bool,
    receive_cc_nrpn: bool,
    pad_dest: ParameterDestination,
    pressure_dest: ParameterDestination,
    encoder_dest: ParameterDestination,
    mute_dest: ParameterDestination,
    ports_output_channel: MidiPortsOutputChannel,

    // Probably `TURBO SPEED` but I can not confirm this.
    #[derivative(Debug = "ignore")]
    pub(crate) __unknown0x29: u8,
}

impl Default for PortConfig {
    fn default() -> Self {
        Self {
            out_port_func: MidiPortFunction::default(),
            thru_port_func: MidiPortFunction::default(),
            input_from: MidiTransportLayer::default(),
            output_to: MidiTransportLayer::default(),
            param_output: MidiParameterOutput::default(),
            receive_notes: true,
            receive_cc_nrpn: true,
            pad_dest: ParameterDestination::default(),
            pressure_dest: ParameterDestination::default(),
            encoder_dest: ParameterDestination::Internal,
            mute_dest: ParameterDestination::default(),
            ports_output_channel: MidiPortsOutputChannel::default(),
            __unknown0x29: 0,
        }
    }
}

impl TryFrom<&ar_global_t> for PortConfig {
    type Error = ConversionError;
    fn try_from(raw_global: &ar_global_t) -> Result<Self, Self::Error> {
        if raw_global.receive_notes > 1 || raw_global.receive_cc_nrpn > 1 {
            return Err(ConversionError::Range {
                value: raw_global.receive_notes.to_string(),
                type_name: "PortConfig".into(),
            });
        }

        Ok(Self {
            out_port_func: raw_global.out_port_func.try_into()?,
            thru_port_func: raw_global.thru_port_func.try_into()?,
            input_from: raw_global.input_from.try_into()?,
            output_to: raw_global.output_to.try_into()?,
            param_output: raw_global.param_output.try_into()?,
            receive_notes: raw_global.receive_notes != 0,
            receive_cc_nrpn: raw_global.receive_cc_nrpn != 0,
            pad_dest: raw_global.pad_dest.try_into()?,
            pressure_dest: raw_global.pressure_dest.try_into()?,
            encoder_dest: raw_global.encoder_dest.try_into()?,
            mute_dest: raw_global.mute_dest.try_into()?,
            ports_output_channel: raw_global.ports_output_channel.try_into()?,
            __unknown0x29: raw_global.__unknown0x29,
        })
    }
}

impl PortConfig {
    pub(crate) fn apply_to_raw_global(&self, raw_global: &mut ar_global_t) {
        raw_global.out_port_func = self.out_port_func.into();
        raw_global.thru_port_func = self.thru_port_func.into();
        raw_global.input_from = self.input_from.into();
        raw_global.output_to = self.output_to.into();
        raw_global.param_output = self.param_output.into();
        raw_global.receive_notes = self.receive_notes as u8;
        raw_global.receive_cc_nrpn = self.receive_cc_nrpn as u8;
        raw_global.pad_dest = self.pad_dest.into();
        raw_global.pressure_dest = self.pressure_dest.into();
        raw_global.encoder_dest = self.encoder_dest.into();
        raw_global.mute_dest = self.mute_dest.into();
        raw_global.ports_output_channel = self.ports_output_channel.into();
    }

    /// Sets the function of the MIDI out port.
    pub fn set_output_port_function(&mut self, output_port_function: MidiPortFunction) {
        self.out_port_func = output_port_function;
    }

    /// Sets the function of the MIDI thru port.
    pub fn set_thru_port_function(&mut self, thru_port_function: MidiPortFunction) {
        self.thru_port_func = thru_port_function;
    }

    /// Sets the transport layer to receive MIDI from.
    pub fn set_input_transport(&mut self, input_from: MidiTransportLayer) {
        self.input_from = input_from;
    }

    /// Sets the transport layer to send MIDI to.
    pub fn set_output_transport(&mut self, output_to: MidiTransportLayer) {
        self.output_to = output_to;
    }

    /// Sets the MIDI parameter output type.
    pub fn set_parameter_output_type(&mut self, param_output: MidiParameterOutput) {
        self.param_output = param_output;
    }

    /// Turns note messages receive on or off.
    pub fn set_receive_notes(&mut self, receive_notes: bool) {
        self.receive_notes = receive_notes;
    }

    /// Turns CC and NRPN messages receive on or off.
    pub fn set_receive_cc_nrpn(&mut self, receive_cc_nrpn: bool) {
        self.receive_cc_nrpn = receive_cc_nrpn;
    }

    /// Sets the destination of parameters produced by pressing pads.
    pub fn set_pad_parameter_destination(
        &mut self,
        pad_parameter_destination: ParameterDestination,
    ) {
        self.pad_dest = pad_parameter_destination;
    }

    /// Sets the destination of parameters produced by pressure amount when pressing pads.
    pub fn set_pressure_parameter_destination(
        &mut self,
        pressure_parameter_destination: ParameterDestination,
    ) {
        self.pressure_dest = pressure_parameter_destination;
    }

    /// Sets the destination of parameters produced by turning encoders.
    pub fn set_encoder_parameter_destination(
        &mut self,
        encoder_parameter_destination: ParameterDestination,
    ) {
        self.encoder_dest = encoder_parameter_destination;
    }

    // TODO: Double check
    /// Sets the destination of parameters produced by muting tracks.
    pub fn set_mute_parameter_destination(
        &mut self,
        mute_parameter_destination: ParameterDestination,
    ) {
        self.mute_dest = mute_parameter_destination;
    }

    /// Sets the channel of the MIDI ports output.
    ///
    /// `AUTO CH` means the channel is determined by the set auto channel.
    /// `TRK_CH` means the channel is determined by the track channel.
    pub fn set_ports_output_channel(&mut self, ports_output_channel: MidiPortsOutputChannel) {
        self.ports_output_channel = ports_output_channel;
    }

    /// Returns the function of the MIDI out port.
    pub fn output_port_function(&self) -> MidiPortFunction {
        self.out_port_func
    }

    /// Returns the function of the MIDI thru port.
    pub fn thru_port_function(&self) -> MidiPortFunction {
        self.thru_port_func
    }

    /// Returns the transport layer to receive MIDI from.
    pub fn input_transport(&self) -> MidiTransportLayer {
        self.input_from
    }

    /// Returns the transport layer to send MIDI to.
    pub fn output_transport(&self) -> MidiTransportLayer {
        self.output_to
    }

    /// Returns the MIDI parameter output type.
    pub fn parameter_output_type(&self) -> MidiParameterOutput {
        self.param_output
    }

    /// Returns `true` if note messages receive is on.
    pub fn receive_notes(&self) -> bool {
        self.receive_notes
    }

    /// Returns `true` if CC and NRPN messages receive is on.
    pub fn receive_cc_nrpn(&self) -> bool {
        self.receive_cc_nrpn
    }

    /// Returns the destination of parameters produced by pressing pads.
    pub fn pad_parameter_destination(&self) -> ParameterDestination {
        self.pad_dest
    }

    /// Returns the destination of parameters produced by pressure amount when pressing pads.
    pub fn pressure_parameter_destination(&self) -> ParameterDestination {
        self.pressure_dest
    }

    /// Returns the destination of parameters produced by turning encoders.
    pub fn encoder_parameter_destination(&self) -> ParameterDestination {
        self.encoder_dest
    }

    /// Returns the destination of parameters produced by muting tracks.
    pub fn mute_parameter_destination(&self) -> ParameterDestination {
        self.mute_dest
    }

    /// Returns the channel of the MIDI ports output.
    pub fn ports_output_channel(&self) -> MidiPortsOutputChannel {
        self.ports_output_channel
    }

    /// Returns `true` if turbo speed is on.
    ///
    /// # Note
    ///
    /// I believe this parameter is `TURBO SPEED` since it is the only one left in the menu when reverse engineering this type.
    /// But since I can not enable it without connecting a turbo speed capable MIDI interface I can not be sure.
    ///
    /// If you have a turbo speed capable MIDI interface and can confirm this please open an issue on the GitHub repository.
    pub fn turbo_speed(&self) -> bool {
        self.__unknown0x29 != 0
    }
}

/// Represents the `Channels` menu in `Midi Config` menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Channels {
    auto_channel: MidiChannel,
    track_channels: [MidiChannel; 12],
    track_fx_channel: MidiChannel,
    program_change_in_channel: MidiChannel,
    program_change_out_channel: MidiChannel,
    performance_channel: MidiChannel,
}

impl Default for Channels {
    // TODO: Double check these defaults.
    fn default() -> Self {
        Self {
            auto_channel: MidiChannel::Channel(13),
            track_channels: [
                MidiChannel::Channel(0),
                MidiChannel::Channel(1),
                MidiChannel::Channel(2),
                MidiChannel::Channel(3),
                MidiChannel::Channel(4),
                MidiChannel::Channel(5),
                MidiChannel::Channel(6),
                MidiChannel::Channel(7),
                MidiChannel::Channel(8),
                MidiChannel::Channel(9),
                MidiChannel::Channel(10),
                MidiChannel::Channel(11),
            ],
            track_fx_channel: MidiChannel::Channel(12),
            program_change_in_channel: MidiChannel::Auto,
            program_change_out_channel: MidiChannel::Auto,
            performance_channel: MidiChannel::Channel(15),
        }
    }
}

impl From<&ar_global_t> for Channels {
    fn from(raw_global: &ar_global_t) -> Self {
        let mut track_channels = [MidiChannel::default(); 12];
        for (i, channel) in raw_global.track_channels.iter().enumerate() {
            track_channels[i] = if *channel == 0xFF {
                MidiChannel::Off
            } else {
                MidiChannel::Channel(*channel as usize)
            };
        }

        Self {
            auto_channel: if raw_global.auto_channel == 0xFF {
                MidiChannel::Off
            } else {
                MidiChannel::Channel(raw_global.auto_channel as usize)
            },
            track_channels,
            track_fx_channel: if raw_global.track_fx_channel == 0xFF {
                MidiChannel::Off
            } else {
                MidiChannel::Channel(raw_global.track_fx_channel as usize)
            },
            program_change_in_channel: if raw_global.prog_ch_in_channel == 0xFF {
                MidiChannel::Auto
            } else {
                MidiChannel::Channel(raw_global.prog_ch_in_channel as usize)
            },
            program_change_out_channel: if raw_global.prog_ch_out_channel == 0xFF {
                MidiChannel::Auto
            } else {
                MidiChannel::Channel(raw_global.prog_ch_out_channel as usize)
            },
            performance_channel: if raw_global.perf_channel == 0xFF {
                MidiChannel::Off
            } else {
                MidiChannel::Channel(raw_global.perf_channel as usize)
            },
        }
    }
}

impl Channels {
    pub(crate) fn apply_to_raw_global(&self, raw_global: &mut ar_global_t) {
        raw_global.auto_channel = self.auto_channel.into();
        raw_global.track_channels[0] = self.track_channels[0].into();
        raw_global.track_channels[1] = self.track_channels[1].into();
        raw_global.track_channels[2] = self.track_channels[2].into();
        raw_global.track_channels[3] = self.track_channels[3].into();
        raw_global.track_channels[4] = self.track_channels[4].into();
        raw_global.track_channels[5] = self.track_channels[5].into();
        raw_global.track_channels[6] = self.track_channels[6].into();
        raw_global.track_channels[7] = self.track_channels[7].into();
        raw_global.track_channels[8] = self.track_channels[8].into();
        raw_global.track_channels[9] = self.track_channels[9].into();
        raw_global.track_channels[10] = self.track_channels[10].into();
        raw_global.track_channels[11] = self.track_channels[11].into();
        raw_global.track_fx_channel = self.track_fx_channel.into();
        raw_global.prog_ch_in_channel = self.program_change_in_channel.into();
        raw_global.prog_ch_out_channel = self.program_change_out_channel.into();
        raw_global.perf_channel = self.performance_channel.into();
    }

    fn validate_midi_channel(channel: &MidiChannel, auto: bool) -> Result<(), ParameterError> {
        // Either auto or off could be used.
        match channel {
            MidiChannel::Off => {
                if auto {
                    Err(ParameterError::Compatibility {
                        value: "MidiChannel::Off".to_string(),
                        parameter_name: "channel".into(),
                        reason: Some("Only MidiChannel::Auto or MidiChannel::Channel(usize) variants are allowed for this function.".into()),
                    })
                } else {
                    Ok(())
                }
            }
            MidiChannel::Auto => {
                if auto {
                    Ok(())
                } else {
                    Err(ParameterError::Compatibility {
                        value: "MidiChannel::Off".to_string(),
                        parameter_name: "channel".into(),
                        reason: Some("Only MidiChannel::Off or MidiChannel::Channel(usize) variants are allowed for this function.".into()),
                    })
                }
            }
            MidiChannel::Channel(channel) => {
                if *channel > 15 {
                    Err(ParameterError::Range {
                        value: channel.to_string(),
                        parameter_name: "MidiChannel".into(),
                    })
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Sets the auto channel.
    ///
    /// Only [`MidiChannel::Off`] or [`MidiChannel::Channel(usize)`] variants can be used.
    ///
    /// Range of [`MidiChannel::Channel(usize)`] is: `0..=15`.
    pub fn set_auto_channel(&mut self, auto_channel: MidiChannel) -> Result<(), RytmError> {
        Self::validate_midi_channel(&auto_channel, false)?;
        self.auto_channel = auto_channel;
        Ok(())
    }

    /// Sets the track channels.
    ///
    /// Only [`MidiChannel::Off`] or [`MidiChannel::Channel(usize)`] variants can be used.
    ///
    /// Range of [`MidiChannel::Channel(usize)`] is: `0..=15`.
    #[parameter_range(range = "track:0..=11")]
    pub fn set_track_channel(
        &mut self,
        track: usize,
        track_channel: MidiChannel,
    ) -> Result<(), RytmError> {
        Self::validate_midi_channel(&track_channel, false)?;
        self.track_channels[track] = track_channel;
        Ok(())
    }

    /// Sets the track FX channel.
    ///
    /// Only [`MidiChannel::Off`] or [`MidiChannel::Channel(usize)`] variants can be used.
    ///
    /// Range of [`MidiChannel::Channel(usize)`] is: `0..=15`.
    pub fn set_track_fx_channel(&mut self, track_fx_channel: MidiChannel) -> Result<(), RytmError> {
        Self::validate_midi_channel(&track_fx_channel, false)?;
        self.track_fx_channel = track_fx_channel;
        Ok(())
    }

    /// Sets the program change in channel.
    ///
    /// Only [`MidiChannel::Auto`] or [`MidiChannel::Channel(usize)`] variants can be used.
    ///
    /// Range of [`MidiChannel::Channel(usize)`] is: `0..=15`.
    pub fn set_program_change_in_channel(
        &mut self,
        program_change_in_channel: MidiChannel,
    ) -> Result<(), RytmError> {
        Self::validate_midi_channel(&program_change_in_channel, true)?;
        self.program_change_in_channel = program_change_in_channel;
        Ok(())
    }

    /// Sets the program change out channel.
    ///
    /// Only [`MidiChannel::Auto`] or [`MidiChannel::Channel(usize)`] variants can be used.
    ///
    /// Range of [`MidiChannel::Channel(usize)`] is: `0..=15`.
    pub fn set_program_change_out_channel(
        &mut self,
        program_change_out_channel: MidiChannel,
    ) -> Result<(), RytmError> {
        Self::validate_midi_channel(&program_change_out_channel, true)?;
        self.program_change_out_channel = program_change_out_channel;
        Ok(())
    }

    /// Sets the performance channel.
    ///
    /// Only [`MidiChannel::Off`] or [`MidiChannel::Channel(usize)`] variants can be used.
    ///
    /// Range of [`MidiChannel::Channel(usize)`] is: `0..=15`.
    pub fn set_performance_channel(
        &mut self,
        performance_channel: MidiChannel,
    ) -> Result<(), RytmError> {
        Self::validate_midi_channel(&performance_channel, false)?;
        self.performance_channel = performance_channel;
        Ok(())
    }

    /// Returns the auto channel.
    pub fn auto_channel(&self) -> MidiChannel {
        self.auto_channel
    }

    /// Returns the track channels.
    pub fn track_channels(&self) -> &[MidiChannel; 12] {
        &self.track_channels
    }

    /// Returns the channel of a track.
    ///
    /// Range: `0..=11`.
    #[parameter_range(range = "track_index:0..=11")]
    pub fn track_channel(&self, track_index: usize) -> Result<MidiChannel, RytmError> {
        Ok(self.track_channels[track_index])
    }

    /// Returns the track FX channel.
    pub fn track_fx_channel(&self) -> MidiChannel {
        self.track_fx_channel
    }

    /// Returns the program change in channel.
    pub fn program_change_in_channel(&self) -> MidiChannel {
        self.program_change_in_channel
    }

    /// Returns the program change out channel.
    pub fn program_change_out_channel(&self) -> MidiChannel {
        self.program_change_out_channel
    }

    /// Returns the performance channel.
    pub fn performance_channel(&self) -> MidiChannel {
        self.performance_channel
    }
}
