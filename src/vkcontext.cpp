#include "vkcontext.h"


// here test
int main() {
    printf("main function started\n");
    vkcontext_t context;
    printf("call started\n");

    auto device = context.get_device();

    vk_compute_pipeline_t compute_pipeline(device.get());
    printf("end of program\n");
}