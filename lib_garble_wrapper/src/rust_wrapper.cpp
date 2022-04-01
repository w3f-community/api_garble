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

// generated
// needed only if shared structs
#include "lib-garble-wrapper/src/lib.rs.h"

#include "garble_helper.h"
#include "packmsg_helper.h"
#include "serialize_packmsg/serialize.h"
#include "serialize_pgc/serialize.h"

using namespace interstellar;

GarbleWrapper::GarbleWrapper() {}

rust::Vec<u_int8_t> GarbleWrapper::GarbleSkcdFromBuffer(rust::Vec<u_int8_t> skcd_buffer) const
{
  // copy rust::Vec -> std::vector
  std::string skcd_buf_cpp;
  std::copy(skcd_buffer.begin(), skcd_buffer.end(), std::back_inserter(skcd_buf_cpp));

  garble::ParallelGarbledCircuit pgc = garble::GarbleSkcdFromBuffer(skcd_buf_cpp);
  std::string pgarbled_buf_cpp = garble::Serialize(pgc);

  rust::Vec<u_int8_t> vec;
  std::copy(pgarbled_buf_cpp.begin(), pgarbled_buf_cpp.end(), std::back_inserter(vec));
  return vec;
}

StrippedCircuit GarbleWrapper::GarbleAndStrippedSkcdFromBuffer(rust::Vec<u_int8_t> skcd_buffer) const
{
  // copy rust::Vec -> std::vector
  std::string skcd_buf_cpp;
  std::copy(skcd_buffer.begin(), skcd_buffer.end(), std::back_inserter(skcd_buf_cpp));

  // TODO return tuple(pgc serialized, pre_packmsg serialized, digits)
  garble::ParallelGarbledCircuit pgc;
  packmsg::PrePackmsg pre_packmsg;
  std::vector<uint8_t> digits;
  packmsg::GarbleAndStrippedSkcdFromBuffer(skcd_buf_cpp, &pgc, &pre_packmsg,
                                           &digits);

  std::string pgarbled_buf_cpp = garble::Serialize(pgc);
  auto prepackmsg_buf_cpp = packmsg::SerializePrepackmsg(pre_packmsg);

  // copy C++ vector -> rust::Vec
  rust::Vec<u_int8_t> pgarbled_buf_vec;
  std::copy(pgarbled_buf_cpp.begin(), pgarbled_buf_cpp.end(), std::back_inserter(pgarbled_buf_vec));
  rust::Vec<u_int8_t> prepackmsg_buf_vec;
  std::copy(prepackmsg_buf_cpp.begin(), prepackmsg_buf_cpp.end(), std::back_inserter(prepackmsg_buf_vec));
  rust::Vec<u_int8_t> digits_vec;
  std::copy(digits.begin(), digits.end(), std::back_inserter(digits_vec));

  StrippedCircuit stripped_circuit;
  stripped_circuit.circuit_buffer = pgarbled_buf_vec;
  stripped_circuit.prepackmsg_buffer = prepackmsg_buf_vec;
  stripped_circuit.digits = digits_vec;
  return stripped_circuit;
}

rust::Vec<u_int8_t> GarbleWrapper::PackmsgFromPrepacket(const rust::Vec<u_int8_t> &prepackmsg_buffer, rust::String message) const
{
  // copy rust::Vec -> std::vector
  std::string prepackmsg_buf_cpp;
  std::copy(prepackmsg_buffer.begin(), prepackmsg_buffer.end(), std::back_inserter(prepackmsg_buf_cpp));

  packmsg::PrePackmsg prepackmsg = packmsg::DeserializePrepackmsgFromBuffer(prepackmsg_buf_cpp);

  std::wstring message_wstring_copy(message.begin(), message.end());
  packmsg::Packmsg packmsg = packmsg::PackmsgFromPrepacket(prepackmsg, message_wstring_copy);

  auto packmsg_buf_cpp = packmsg::SerializePackmsg(packmsg);

  rust::Vec<u_int8_t> vec;
  std::copy(packmsg_buf_cpp.begin(), packmsg_buf_cpp.end(), std::back_inserter(vec));
  return vec;
}

std::unique_ptr<GarbleWrapper> new_garble_wrapper()
{
  return std::make_unique<GarbleWrapper>();
}