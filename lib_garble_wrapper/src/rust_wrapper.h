// api_garble
// Copyright (C) 2O22  Nathan Prat

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// wrapper for our "lib_server" circuit generator

#pragma once

#include <memory>

#include "rust/cxx.h"

// rust-cxx shared struct
struct StrippedCircuit;

/**
 * Wrapper around interstellar::garblehelper::GarbleHelper
 */
class GarbleWrapper
{
public:
  GarbleWrapper();

  rust::Vec<u_int8_t> GarbleSkcdFromBuffer(rust::Vec<u_int8_t> skcd_buffer) const;

  /**
   * return a buffer containing a Protobuf-serialized Prepackmsg
   * It can later be used to create a Packmsg with a given tx message,
   * then finally be sent to a device allow the PGC to be evaluated.
   */
  StrippedCircuit GarbleAndStrippedSkcdFromBuffer(rust::Vec<u_int8_t> skcd_buffer) const;

  /**
   * param: prepackmsg_buffer: a Prepackmsg = the returned value from "GarbleAndStrippedSkcdFromBuffer"
   * param:
   */
  rust::Vec<u_int8_t> PackmsgFromPrepacket(const rust::Vec<u_int8_t> &prepackmsg_buffer, rust::String message) const;
};

std::unique_ptr<GarbleWrapper> new_garble_wrapper();