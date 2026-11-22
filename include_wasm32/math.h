#pragma once

#define sqrtf(f) __builtin_sqrtf(f)
#define fabsf(f) __builtin_fabsf(f)
#define floorf(f) __builtin_floorf(f)
#define ceilf(f) __builtin_ceilf(f)
#define log2f(f) __builtin_log2f(f)
#define cosf(f) __builtin_cosf(f)
#define ldexpf(f, e) __builtin_ldexpf(f, e)

// frexpf has no WebAssembly instruction, and no library provides it to wasm32 modules (Rust's
// compiler-builtins has ldexpf, log2f and cosf, but not frexpf), so it is defined here: the value
// split into a mantissa in [0.5, 1) and a power of two, as the standard frexpf does.
static inline float meshopt_wasm32_frexpf(float value, int* exponent)
{
	unsigned int bits;
	__builtin_memcpy(&bits, &value, sizeof(bits));
	int biased = int((bits >> 23) & 0xff);

	if (biased == 0xff || value == 0.f)
	{
		// infinities, NaNs and zeros are returned as they are
		*exponent = 0;
		return value;
	}

	int offset = 0;
	if (biased == 0)
	{
		// a subnormal value is scaled by 2^25 into the normal range first
		value *= 33554432.f;
		__builtin_memcpy(&bits, &value, sizeof(bits));
		biased = int((bits >> 23) & 0xff);
		offset = 25;
	}

	*exponent = biased - 126 - offset;
	bits = (bits & 0x807fffffu) | 0x3f000000u;
	__builtin_memcpy(&value, &bits, sizeof(value));
	return value;
}

#define frexpf(f, e) meshopt_wasm32_frexpf(f, e)
