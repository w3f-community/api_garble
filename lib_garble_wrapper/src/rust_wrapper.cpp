#include "rust_wrapper.h"

#include <functional>

#include "garble_helper.h"

GarbleWrapper::GarbleWrapper() {}

rust::Vec<u_int8_t> GarbleWrapper::GarbleSkcdToBuffer(rust::Str skcd_input_path) const
{
  std::string buf = interstellar::garblehelper::GarbleSkcdToBuffer(std::string(skcd_input_path));

  rust::Vec<u_int8_t> vec;
  std::copy(buf.begin(), buf.end(), std::back_inserter(vec));
  return vec;
}

std::unique_ptr<GarbleWrapper> new_garble_wrapper()
{
  return std::make_unique<GarbleWrapper>();
}