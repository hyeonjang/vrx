#include "vkcontext.h"


// here test
int main() {
    printf("main function started\n");
    vx::vkcontext_t context;
    printf("call started\n");

    vx::Pipeline compute_pipeline(&context.device);
    // printf("end of program\n");
}