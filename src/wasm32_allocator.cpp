// operator new and operator delete for wasm32, which has no C++ standard library to provide them:
// they forward to the Rust global allocator (src/wasm32_allocator.rs). meshoptimizer allocates
// its temporary buffers through them unless meshopt_setAllocator is called.
#include <stddef.h>

extern "C" void* meshopt_rust_allocate(size_t size);
extern "C" void meshopt_rust_deallocate(void* pointer);

void* operator new(size_t size)
{
	return meshopt_rust_allocate(size);
}

void operator delete(void* pointer) noexcept
{
	meshopt_rust_deallocate(pointer);
}
