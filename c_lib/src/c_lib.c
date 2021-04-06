/**
 * @copyright Copyright © 2021, KeepTruckin, Inc.
 * All rights reserved.
 * All information contained herein is the property of KeepTruckin Inc. The
 * intellectual and technical concepts contained herein are proprietary to
 * KeepTruckin. Dissemination of this information or reproduction of this
 * material is strictly forbidden unless prior written permission is obtained
 * from KeepTruckin.
 * @file   c_lib.c
 * @date   2021-04-01
 * @brief 
 */
#include <stdio.h>

#include "c_lib.h"

int c_double_input(int32_t input)
{
    // Integer overflow is undefined behavior.
    if (input > (INT32_MAX / 2))
    {
        printf("Error, input too large!\n");
        return 0;
    }
    return (2 * input);
}

void c_increment_int_array(size_t length, int32_t array[])
{
    for (size_t i = 0; i < length; i++) {
        if (array[i] >= (INT32_MAX - 1))
        {
            printf("Error, an input array value is too large!\n");
            return;
        }
        array[i]++;
    }
}
