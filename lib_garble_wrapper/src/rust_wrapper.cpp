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

#include "rust_wrapper.h"

#include <functional>

#include "garble_helper.h"

GarbleWrapper::GarbleWrapper() {}

rust::Vec<u_int8_t> GarbleWrapper::GarbleSkcdFromBufferToBuffer(rust::Vec<u_int8_t> skcd_buffer) const
{
  std::string skcd_buf_cpp;
  std::copy(skcd_buffer.begin(), skcd_buffer.end(), std::back_inserter(skcd_buf_cpp));
  std::string buf_cpp = interstellar::garblehelper::GarbleSkcdFromBufferToBuffer(skcd_buf_cpp);

  rust::Vec<u_int8_t> vec;
  std::copy(buf_cpp.begin(), buf_cpp.end(), std::back_inserter(vec));
  return vec;
}

// TODO
#if 0
rust::Vec<u_int8_t> GarbleWrapper::GarbleAndStrippedSkcdFromBufferToBuffer(rust::Vec<u_int8_t> skcd_buffer) const
{
  std::string skcd_buf_cpp;
  std::copy(skcd_buffer.begin(), skcd_buffer.end(), std::back_inserter(skcd_buf_cpp));
  std::string buf_cpp = interstellar::garblehelper::GarbleSkcdFromBufferToBuffer(skcd_buf_cpp);

  rust::Vec<u_int8_t> vec;
  std::copy(buf_cpp.begin(), buf_cpp.end(), std::back_inserter(vec));
  return vec;
}
#endif

std::unique_ptr<GarbleWrapper> new_garble_wrapper()
{
  return std::make_unique<GarbleWrapper>();
}