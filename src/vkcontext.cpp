#include "vkcontext.h"


// here test
int main() {
    printf("main function started\n");
    vkcontext_t context;
    printf("call started\n");

    vk_pipeline_t compute_pipeline(&context.device, &context.allocator);
    // printf("end of program\n");
}