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
#include "packmsg_helper.h"
#include "serialize_packmsg/serialize.h"
#include "serialize_pgc/serialize.h"

GarbleWrapper::GarbleWrapper() {}

rust::Vec<u_int8_t> GarbleWrapper::GarbleSkcdFromBuffer(rust::Vec<u_int8_t> skcd_buffer) const
{
  // copy rust::Vec -> std::vector
  std::string skcd_buf_cpp;
  std::copy(skcd_buffer.begin(), skcd_buffer.end(), std::back_inserter(skcd_buf_cpp));

  interstellar::garble::ParallelGarbledCircuit pgc = interstellar::garble::GarbleSkcdFromBuffer(skcd_buf_cpp);
  std::string pgarbled_buf_cpp = interstellar::garble::Serialize(pgc);

  rust::Vec<u_int8_t> vec;
  std::copy(pgarbled_buf_cpp.begin(), pgarbled_buf_cpp.end(), std::back_inserter(vec));
  return vec;
}

rust::Vec<u_int8_t> GarbleWrapper::GarbleAndStrippedSkcdFromBuffer(rust::Vec<u_int8_t> skcd_buffer) const
{
  // copy rust::Vec -> std::vector
  std::string skcd_buf_cpp;
  std::copy(skcd_buffer.begin(), skcd_buffer.end(), std::back_inserter(skcd_buf_cpp));

  // TODO return tuple(pgc serialized, pre_packmsg serialized, digits)
  interstellar::garble::ParallelGarbledCircuit pgc;
  interstellar::packmsg::PrePackmsg pre_packmsg;
  std::vector<uint8_t> digits;
  interstellar::packmsg::GarbleAndStrippedSkcdFromBuffer(skcd_buf_cpp, &pgc, &pre_packmsg,
                                                         &digits);

  auto prepackmsg_buf_cpp = interstellar::packmsg::Serialize(pre_packmsg);

  rust::Vec<u_int8_t> vec;
  std::copy(prepackmsg_buf_cpp.begin(), prepackmsg_buf_cpp.end(), std::back_inserter(vec));
  return vec;
}

std::unique_ptr<GarbleWrapper> new_garble_wrapper()
{
  return std::make_unique<GarbleWrapper>();
}