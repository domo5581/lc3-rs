/*
read lc3 asm file
let contents = std::fs::read_to_string(filename);
load into symbol table
for word in contents
iter through symbol table and write hex codes to obj file
*/

pub struct Assembler {
    pub Tokens: Vec<String>,
    
}