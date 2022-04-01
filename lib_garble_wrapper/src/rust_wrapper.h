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

/**
 * Wrapper around interstellar::garblehelper::GarbleHelper
 */
class GarbleWrapper
{
public:
  GarbleWrapper();

  rust::Vec<u_int8_t> GarbleSkcdFromBufferToBuffer(rust::Vec<u_int8_t> skcd_buffer) const;
};

std::unique_ptr<GarbleWrapper> new_garble_wrapper();