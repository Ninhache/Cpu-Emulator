## Fourth prototype.. that quite long

## Objectives

Re-use the opcode (in the last version they got removed for no reason)

the final version need to have something similar to this :

```
         |         assembler           |             |        CPU       |
asm file > parse asm > gen binary file > binary file > decode > execute
```

The last version where like this :

```
        |        CPU                   |
asm file > parse asm > decode > execute
```

I've abstract too much layers, i've to restart again ahaha
