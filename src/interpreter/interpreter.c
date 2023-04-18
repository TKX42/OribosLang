#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NELEMS(x)  (sizeof(x) / sizeof((x)[0]))
#define STACK_SIZE 256
#define MEM_SIZE 1024

enum DataType {
    INT,
    FLOAT,
    STRING,
    NIL
};

struct string {
    void *data;
};

struct Data {
    enum DataType type;
    union {
        int integer;
        float floating;
        struct string string;
    } data;
};

void runtime_error(char *msg) {
    printf("Runtime Error: %s", msg);
    exit(1);
}

void print_str(struct string *string) {
    printf("%s\n", (char *) string->data);
}

void print(struct Data *data) {
    switch (data->type) {
        case INT:
            printf("%d\n", data->data.integer);
            break;
        case FLOAT:
            printf("%f\n", data->data.floating);
            break;
        case STRING:
            print_str(&data->data.string);
            break;
        case NIL:
            runtime_error("Cannot print NIL");
            break;
    }
}

enum instr_type {
    LOAD,
    ASSIGN,
    CONST,
    PRINT
};

struct instr {
    enum instr_type type;
    struct Data data;
};

void sc_check(const int sc, const int size) {
    if (sc < 0 || sc >= size) {
        runtime_error("Invalid stack counter");
    }
}

void instr_run(struct instr *instruction, struct Data *stack, int *sc, struct Data *memory) {
    switch (instruction->type) {
        case LOAD: {
            (*sc)++;
            struct Data data = memory[instruction->data.data.integer];
            stack[*sc] = data;
            break;
        }
        case ASSIGN: {
            sc_check(*sc, STACK_SIZE);
            struct Data data = stack[*sc];
            memory[instruction->data.data.integer] = data;
            (*sc)--;
            break;
        }
        case CONST: {
            (*sc)++;
            stack[*sc] = instruction->data;
            break;
        }
        case PRINT: {
            sc_check(*sc, STACK_SIZE);
            struct Data data = stack[*sc];
            print(&data);
            (*sc)--;
            break;
        }
    }
}

void vm_run(struct instr *instructions, size_t len, struct Data *stack, struct Data *memory) {
    int pc = 0;
    int sc = -1;
    while (pc < len) {
        instr_run(&instructions[pc], stack, &sc, memory);
        pc++;
    }
}

char* parse(char* code) {
    char delim[] = " ";
    char* x = strtok(code, delim);
    int init_size = strlen(code);

    char *ptr = strtok(code, delim);
    while (ptr != NULL)
    {
        printf("'%s'\n", ptr);
        ptr = strtok(NULL, delim);
    }

    return malloc(4);
}

int main() {
    printf("ORIBOS_C\n");

    struct Data stack[STACK_SIZE];
    struct Data memory[MEM_SIZE];

    char* demo_code = "PUSH 42"
                      "MOVE 0";
                      "LOAD 0";
                      "SOUT";

    char* instrs = parse(demo_code);
    free(instrs);

    struct instr instr_1 = {CONST, {INT, {.integer=42}}};
    struct instr instr_2 = {ASSIGN, {INT, {.integer=0}}};
    struct instr instr_3 = {CONST, {STRING, {.string="the end"}}};
    struct instr instr_4 = {LOAD, {INT, {.integer=0}}};
    struct instr instr_5 = {PRINT, {NIL}};
    struct instr instr_6 = {PRINT, {NIL}};

    struct instr instructions[] = {instr_1, instr_2, instr_3, instr_4, instr_5, instr_6};

    vm_run((struct instr *) &instructions, NELEMS(instructions), stack, memory);

    return 0;
}
