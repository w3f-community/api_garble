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

  rust::Vec<u_int8_t> GarbleSkcdToBuffer(rust::Str output_skcd_path) const;
};

std::unique_ptr<GarbleWrapper> new_garble_wrapper();