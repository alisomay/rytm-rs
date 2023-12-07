use crate::{
    error::{ParameterError, RytmError},
    global::types::{RoutingUsbInOptions, RoutingUsbOutOptions, RoutingUsbToMainDb},
};
use rytm_rs_macro::parameter_range;
use rytm_sys::ar_global_t;

/// Represents the routing menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Routing {
    route_to_main_flags: u16,
    send_to_fx_flags: u16,
    usb_in: RoutingUsbInOptions,
    usb_out: RoutingUsbOutOptions,
    usb_to_main_db: RoutingUsbToMainDb,
}

impl Default for Routing {
    fn default() -> Self {
        Self {
            route_to_main_flags: 0x0FFF,
            send_to_fx_flags: 0x0FFF,
            usb_in: RoutingUsbInOptions::default(),
            usb_out: RoutingUsbOutOptions::default(),
            usb_to_main_db: RoutingUsbToMainDb::default(),
        }
    }
}

impl TryFrom<&ar_global_t> for Routing {
    type Error = RytmError;
    fn try_from(raw_global: &ar_global_t) -> Result<Self, Self::Error> {
        Ok(Self {
            route_to_main_flags: ((raw_global.route_to_main_msb as u16) << 8)
                | raw_global.route_to_main_lsb as u16,
            send_to_fx_flags: ((raw_global.send_to_fx_msb as u16) << 8)
                | raw_global.send_to_fx_lsb as u16,
            usb_in: raw_global.usb_in.try_into()?,
            usb_out: raw_global.usb_out.try_into()?,
            usb_to_main_db: raw_global.usb_to_main_db.try_into()?,
        })
    }
}

impl Routing {
    pub(crate) fn apply_to_raw_global(&self, raw_global: &mut ar_global_t) {
        raw_global.route_to_main_msb = (self.route_to_main_flags >> 8) as u8;
        raw_global.route_to_main_lsb = self.route_to_main_flags as u8;
        raw_global.send_to_fx_msb = (self.send_to_fx_flags >> 8) as u8;
        raw_global.send_to_fx_lsb = self.send_to_fx_flags as u8;
        raw_global.usb_in = self.usb_in.into();
        raw_global.usb_out = self.usb_out.into();
        raw_global.usb_to_main_db = self.usb_to_main_db.into();
    }

    /// Sets the routing flags for `ROUTE TO MAIN`.
    ///
    /// Range `0..=0b1111_1111_1111`
    ///
    /// # Example
    ///
    /// - `0` would route no tracks to main.
    /// - `0b000000000001` would route track 0 to main.
    /// - `0xFFF`would route all tracks to main.
    #[parameter_range(range = "route_to_main_flags:0..=4095")]
    pub fn set_route_to_main_flags(&mut self, route_to_main_flags: u16) -> Result<(), RytmError> {
        self.route_to_main_flags = route_to_main_flags;
        Ok(())
    }

    /// Routes a track to main.
    ///
    /// Range `0..=11`
    #[parameter_range(range = "track_index:0..=11")]
    pub fn route_track_to_main(&mut self, track_index: usize) -> Result<(), RytmError> {
        self.route_to_main_flags |= 1 << track_index;
        Ok(())
    }

    /// Clears the routing of a track to main.
    ///
    /// Range `0..=11`
    #[parameter_range(range = "track_index:0..=11")]
    pub fn clear_routing_of_a_track_to_main(
        &mut self,
        track_index: usize,
    ) -> Result<(), RytmError> {
        self.route_to_main_flags &= !(1 << track_index);
        Ok(())
    }

    /// Sets the routing flags for `SEND TO FX`.
    ///
    /// Range `0..=0b1111_1111_1111`
    ///
    /// # Example
    ///
    /// - `0` would send no tracks to FX.
    /// - `0b000000000001` would send track 0 to FX.
    /// - `0xFFF`would send all tracks to FX.
    #[parameter_range(range = "send_to_fx_flags:0..=4095")]
    pub fn set_send_to_fx_flags(&mut self, send_to_fx_flags: u16) -> Result<(), RytmError> {
        self.send_to_fx_flags = send_to_fx_flags;
        Ok(())
    }

    /// Sends a track to FX.
    ///
    /// Range `0..=11`
    #[parameter_range(range = "track_index:0..=11")]
    pub fn send_track_to_fx(&mut self, track_index: usize) -> Result<(), RytmError> {
        self.send_to_fx_flags |= 1 << track_index;
        Ok(())
    }

    /// Clears the sending of a track to FX.
    ///
    /// Range `0..=11`
    #[parameter_range(range = "track_index:0..=11")]
    pub fn clear_sending_of_a_track_to_fx(&mut self, track_index: usize) -> Result<(), RytmError> {
        self.send_to_fx_flags &= !(1 << track_index);
        Ok(())
    }

    /// Sets the `USB IN` routing.
    pub fn set_usb_in(&mut self, usb_in: RoutingUsbInOptions) -> Result<(), RytmError> {
        self.usb_in = usb_in;
        Ok(())
    }

    /// Sets the `USB OUT` routing.
    pub fn set_usb_out(&mut self, usb_out: RoutingUsbOutOptions) -> Result<(), RytmError> {
        self.usb_out = usb_out;
        Ok(())
    }

    /// Sets the `USB TO MAIN [dB]` routing.
    pub fn set_usb_to_main_db(
        &mut self,
        usb_to_main_db: RoutingUsbToMainDb,
    ) -> Result<(), RytmError> {
        self.usb_to_main_db = usb_to_main_db;
        Ok(())
    }

    /// Returns the raw `ROUTE TO MAIN` flags.
    pub fn raw_route_to_main_flags(&self) -> u16 {
        self.route_to_main_flags
    }

    /// Returns the track indexes routed to main.
    pub fn track_indexes_routed_to_main(&self) -> Vec<usize> {
        let mut routed = Vec::new();
        for track_index in 0..=11 {
            if self.route_to_main_flags & (1 << track_index) != 0 {
                routed.push(track_index);
            }
        }
        routed
    }

    /// Returns the track indexes not routed to main.
    pub fn track_indexes_not_routed_to_main(&self) -> Vec<usize> {
        let mut not_routed = Vec::new();
        for track_index in 0..=11 {
            if self.route_to_main_flags & (1 << track_index) == 0 {
                not_routed.push(track_index);
            }
        }
        not_routed
    }

    /// Returns `true` if a track is routed to main.
    pub fn is_track_routed_to_main(&self, track_index: usize) -> bool {
        self.route_to_main_flags & (1 << track_index) != 0
    }

    /// Returns the raw `SEND TO FX` flags.
    pub fn raw_send_to_fx_flags(&self) -> u16 {
        self.send_to_fx_flags
    }

    /// Returns the track indexes sent to FX.
    pub fn track_indexes_sent_to_fx(&self) -> Vec<usize> {
        let mut sent = Vec::new();
        for track_index in 0..=11 {
            if self.send_to_fx_flags & (1 << track_index) != 0 {
                sent.push(track_index);
            }
        }
        sent
    }

    /// Returns the track indexes not sent to FX.
    pub fn track_indexes_not_sent_to_fx(&self) -> Vec<usize> {
        let mut not_sent = Vec::new();
        for track_index in 0..=11 {
            if self.send_to_fx_flags & (1 << track_index) == 0 {
                not_sent.push(track_index);
            }
        }
        not_sent
    }

    /// Returns `true` if a track is sent to FX.
    pub fn is_track_sent_to_fx(&self, track_index: usize) -> bool {
        self.send_to_fx_flags & (1 << track_index) != 0
    }

    /// Returns the `USB IN` routing configuration.
    pub fn usb_in(&self) -> RoutingUsbInOptions {
        self.usb_in
    }

    /// Returns the `USB OUT` routing configuration.
    pub fn usb_out(&self) -> RoutingUsbOutOptions {
        self.usb_out
    }

    /// Returns the `USB TO MAIN [dB]` value.
    pub fn usb_to_main_db(&self) -> RoutingUsbToMainDb {
        self.usb_to_main_db
    }
}
