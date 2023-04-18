#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>

#define NELEMS(x)  (sizeof(x) / sizeof((x)[0]))
#define STREQ(a, b) (strcmp(a,b)==0)
#define INSTR_EQ(x) (STREQ(instr_type, x))

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

void init_error(char *msg) {
    printf("Init Error: %s", msg);
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
            printf("NIL");
            break;
    }
}

enum instr_type {
    LOAD,
    ASSIGN,
    CONST,
    IFJUMP,
    JUMP,
    ADD,
    MOD,
    EQ,
    LESS,
    PRINT
};

struct instr {
    enum instr_type type;
    struct Data data;
};

void sc_pop_check(const int sc, const int size) {
    if (sc < 0 || sc >= size) {
        runtime_error("Invalid stack counter");
    }
}

void sc_push_check(const int sc, const int size) {
    if (sc >= size) {
        runtime_error("Invalid stack counter");
    }
}

void instr_run(struct instr *instruction, int *pc, struct Data *stack, int *sc, struct Data *memory) {
    switch (instruction->type) {
        case LOAD: {
            (*sc)++;
            sc_push_check(*sc, STACK_SIZE);
            struct Data data = memory[instruction->data.data.integer];
            stack[*sc] = data;
            break;
        }
        case ASSIGN: {
            sc_pop_check(*sc, STACK_SIZE);
            struct Data data = stack[*sc];
            memory[instruction->data.data.integer] = data;
            (*sc)--;
            break;
        }
        case CONST: {
            (*sc)++;
            sc_push_check(*sc, STACK_SIZE);
            stack[*sc] = instruction->data;
            break;
        }
        case IFJUMP: {
            sc_pop_check(*sc, STACK_SIZE);
            int condition = stack[*sc].data.integer;
            if (condition == 1) {
                (*pc) += instruction->data.data.integer -
                         1;  // -1 because pc will be increased again in vm_run after an instruction completed
            }
            (*sc)--;
            break;
        }
        case JUMP: {
            (*pc) += instruction->data.data.integer - 1;
            break;
        }
        case PRINT: {
            sc_pop_check(*sc, STACK_SIZE);
            struct Data data = stack[*sc];
            print(&data);
            (*sc)--;
            break;
        }
        case ADD: {
            int right = stack[*sc].data.integer;
            (*sc)--;
            int left = stack[*sc].data.integer;
            struct Data result = {INT, .data={.integer=left + right}};
            stack[*sc] = result;
            break;
        }
        case MOD: {
            int right = stack[*sc].data.integer;
            (*sc)--;
            int left = stack[*sc].data.integer;
            struct Data result = {INT, .data={.integer=left % right}};
            stack[*sc] = result;
            break;
        }
        case EQ: {
            int right = stack[*sc].data.integer;
            (*sc)--;
            int left = stack[*sc].data.integer;
            int eq = left == right;
            struct Data result = {INT, .data={.integer=eq}};
            stack[*sc] = result;
            break;
        }
        case LESS: {
            int right = stack[*sc].data.integer;
            (*sc)--;
            int left = stack[*sc].data.integer;
            struct Data result = {INT, .data={.integer=left <= right}};
            stack[*sc] = result;
            break;
        }
    }
}

void vm_run(struct instr *instructions, size_t len, struct Data *stack, struct Data *memory) {
    int pc = 0;
    int sc = -1;
    while (pc < len) {
        instr_run(&instructions[pc], &pc, stack, &sc, memory);
        pc++;
    }
}

int count_string(const char *haystack, const char *needle) {
    int count = 0;
    const char *tmp = haystack;
    while (tmp = strstr(tmp, needle)) {
        count++;
        tmp++;
    }
    return count;
}

struct instr parse_instr(char *instr_type, char *parameter) {
    if (INSTR_EQ("PRINT")) {
        struct instr instr = {PRINT, {NIL}};
        return instr;
    }

    if (INSTR_EQ("CONST")) {
        int data = strtol(parameter, NULL, 0);
        struct instr instr = {CONST, {.type=INT, .data.integer=data}};
        return instr;
    }

    if (INSTR_EQ("LOAD")) {
        int p = strtol(parameter, NULL, 0);
        struct instr instr = {LOAD, {.type=INT, .data.integer=p}};
        return instr;
    }

    if (INSTR_EQ("ASSIGN")) {
        int p = strtol(parameter, NULL, 0);
        struct instr instr = {ASSIGN, {.type=INT, .data.integer=p}};
        return instr;
    }

    if (INSTR_EQ("IFJUMP")) {
        int p = strtol(parameter, NULL, 0);
        struct instr instr = {IFJUMP, {.type=INT, .data.integer=p}};
        return instr;
    }

    if (INSTR_EQ("JUMP")) {
        int p = strtol(parameter, NULL, 0);
        struct instr instr = {JUMP, {.type=INT, .data.integer=p}};
        return instr;
    }

    if (INSTR_EQ("ADD")) {
        struct instr instr = {ADD, {.type=NIL}};
        return instr;
    }

    if (INSTR_EQ("MOD")) {
        struct instr instr = {MOD, {.type=NIL}};
        return instr;
    }

    if(INSTR_EQ("EQ")) {
        struct instr instr = {EQ, {.type=NIL}};
        return instr;
    }

    if(INSTR_EQ("LESS")) {
        struct instr instr = {LESS, {.type=NIL}};
        return instr;
    }

    init_error("Unknown instruction");
    // unreachable
}

void parse(const char *code, struct instr *instructions) {
    int i = 0;
    char *token = strtok(code, "\n");
    while (token != NULL) {
        size_t max_cap = strlen(token);
        char instruction[max_cap];
        char parameter[max_cap];
        sscanf(token, "%s %s", instruction, parameter);
        instructions[i] = parse_instr((char *) &instruction, parameter);

        token = strtok(NULL, "\n");
        i++;
    }
}

int main() {
    printf("ORIBOS_C\n");

    struct Data stack[STACK_SIZE];
    struct Data memory[MEM_SIZE];

    char demo_code[] = "CONST 370248451\n"
                       "ASSIGN 1\n"
                       "LOAD 1\n"
                       "PRINT\n"
                       "CONST 0\n"
                       "ASSIGN 2\n"
                       "LOAD 1\n"
                       "CONST 1\n"
                       "EQ\n"
                       "IFJUMP 2\n"
                       "JUMP 4\n"
                       "CONST -111\n"
                       "PRINT\n"
                       "JUMP 31\n"
                       "CONST 2\n"
                       "ASSIGN 3\n"
                       "LOAD 1\n"
                       "CONST 2\n"
                       "MOD\n"
                       "CONST 0\n"
                       "EQ\n"
                       "IFJUMP 2\n"
                       "JUMP 4\n"
                       "CONST 1\n"
                       "ASSIGN 2\n"
                       "JUMP 1\n"
                       "LOAD 3\n"
                       "CONST 1\n"
                       "ADD\n"
                       "ASSIGN 3\n"
                       "LOAD 3\n"
                       "LOAD 1\n"
                       "LESS\n"
                       "IFJUMP -17\n"
                       "LOAD 2\n"
                       "CONST 1\n"
                       "EQ\n"
                       "IFJUMP 2\n"
                       "JUMP 4\n"
                       "CONST 0000\n"
                       "PRINT\n"
                       "JUMP 3\n"
                       "CONST 1111\n"
                       "PRINT\n";

    int count = count_string(demo_code, "\n");
    struct instr instrs[count];
    parse((const char *) &demo_code, instrs);
    vm_run(instrs, count, stack, memory);

    return 0;
}
