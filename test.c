#include <stdlib.h>
#include <stdio.h>

struct Callbacks {
	void* (*create_instance)();
	void (*destroy_instance)(void* ptr);
	void (*update_instance)(void* ptr);
};

extern struct Callbacks g_backend;

int main() {
	void* instance = g_backend.create_instance();
	g_backend.update_instance(instance);
	g_backend.update_instance(instance);
	g_backend.update_instance(instance);
	return 0;
}
