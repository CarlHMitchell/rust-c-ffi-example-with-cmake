#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>

#include "rust_lib.h"

int main()
{
    printf("Hello, World from C!\n");
    println_hello_world();
    char* greeting = return_hello_world();
    printf("%s", greeting);
    free_rust_allocated_string(greeting);
    greeting = NULL;
    printf("\n");

    const char* stringWithHotdog = "This string has \"hotdog\" in it";
    const char* stringWithoutHotdog = "This string doesn't have the forbidden food";
    if (contains_hotdog(stringWithHotdog))
    {
        printf("\"%s\" contains \"hotdog\"\n", stringWithHotdog);
    }
    if (!contains_hotdog(stringWithoutHotdog))
    {
        printf("\"%s\" doesn't contain \"hotdog\"\n", stringWithoutHotdog);
    }
    printf("\n");

    int input = 4;
    int output = double_input(input);
    printf("%d * 2 = %d\n", input, output);
    printf("\n");

    const char* testString = "göes to élevên";
    uint32_t count = how_many_characters(testString);
    uint32_t len = how_many_bytes(testString);
    printf("Rust count() counts Unicode Extended Grapheme Clusters, Rust len() counts bytes, C strlen() counts bytes.\n");
    printf("in \"%s\", rust count() = %" PRIu32 ", rust len() = %" PRIu32 ", C strlen() = %" PRIu64 "\n", testString, count, len, strlen(testString));
    printf("\n");

    uint32_t numbers[] = {1, 2, 3, 4, 5, 6};
    size_t length = sizeof numbers / sizeof *numbers;
    uint32_t sum = sum_of_even(numbers, length);
    printf("sum_of_even from 1-6: %" PRIu32 "\n", sum);

    Tuple initial = { .x = 10, .y = 20 };
    Tuple result = flip_things_around(initial);
    printf("Initial tuple (a,b): (%" PRIu32 ", %" PRIu32 "). (b+1, a-1): (%" PRIu32 ",%" PRIu32 ")\n",initial.x, initial.y, result.x, result.y);
    printf("\n");

    ZipCodeDatabase* database = zip_code_database_new();
    zip_code_database_populate(database);
    uint32_t pop1 = zip_code_database_population_of(database, "90210");
    uint32_t pop2 = zip_code_database_population_of(database, "20500");
    zip_code_database_free(database);
    printf("Object/method example 90210 - 20500 = %" PRId32 "\n", (int32_t)pop1 - (int32_t)pop2);
    printf("\n");

    #define INCREMENT_LIST_LENGTH (8U)
    int32_t numbers_to_increment[INCREMENT_LIST_LENGTH] = {0, 1, 2, 3, 4, 5, 6, 7};
    printf("To increment: ");
    for (size_t i = 0; i < INCREMENT_LIST_LENGTH; i++) {
        printf("%d, ", numbers_to_increment[i]);
    }
    printf("\nIncremented via Rust: ");
    increment_array(INCREMENT_LIST_LENGTH, numbers_to_increment);
    for (size_t i = 0; i < INCREMENT_LIST_LENGTH; i++) {
        printf("%d, ", numbers_to_increment[i]);
    }
    printf("\nIncremented again, via Rust via C: ");
    increment_array_via_c(INCREMENT_LIST_LENGTH, numbers_to_increment);
    for (size_t i = 0; i < INCREMENT_LIST_LENGTH; i++) {
        printf("%d, ", numbers_to_increment[i]);
    }
    printf("\n");

    return 0;
}
