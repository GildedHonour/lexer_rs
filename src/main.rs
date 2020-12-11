const STACK_DEPTH: i32 = 2048;
const DEFAULT_TEMPLATE_DEPTH: i32 = 65536;
const STANDARD_IMPORT: i32  = 0x1;
const IMPORT_META: i32 = 0x2;

struct ImportStatement {
  start: u16,
  end: u16,
  statement_start: u16,
  statement_end: u16,
  dynamic_: u16,
  // struct ImportStatement* next;
}

struct ExportStatement {
  start: u16,
  end: u16,
  // struct ExportStatement next;
}



fn parse() {

}


fn try_parse_import_statement() {

}

fn try_parse_export_statement() {

}


fn is_expression_keyword(curr_pos: &str) -> bool {
  let char_vec: Vec<char> = curr_pos.chars().collect();
  let ch = char_vec[0];


  match ch {
    'd' =>
      switch (*(pos - 1)) {
        case 'i':
          // void
          return readPrecedingKeyword2(pos - 2, 'v', 'o');
        case 'l':
          // yield
          return readPrecedingKeyword3(pos - 2, 'y', 'i', 'e');
        default:
          return false;
      }
    'e' =>
      switch (*(pos - 1)) {
        case 's':
          switch (*(pos - 2)) {
            case 'l':
              // else
              return readPrecedingKeyword1(pos - 3, 'e');
            case 'a':
              // case
              return readPrecedingKeyword1(pos - 3, 'c');
            default:
              return false;
          }
        case 't':
          // delete
          return readPrecedingKeyword4(pos - 2, 'd', 'e', 'l', 'e');
        default:
          return false;
      }
    'f' =>
      if (*(pos - 1) != 'o' || *(pos - 2) != 'e')
        return false;
      switch (*(pos - 3)) {
        case 'c':
          // instanceof
          return readPrecedingKeyword6(pos - 4, 'i', 'n', 's', 't', 'a', 'n');
        case 'p':
          // typeof
          return readPrecedingKeyword2(pos - 4, 't', 'y');
        default:
          return false;
      }
    'n' =>
      // in, return
      return readPrecedingKeyword1(pos - 1, 'i') || readPrecedingKeyword5(pos - 1, 'r', 'e', 't', 'u', 'r');
    'o' =>
      // do
      return readPrecedingKeyword1(pos - 1, 'd');
    'r' =>
      // debugger
      return readPrecedingKeyword7(pos - 1, 'd', 'e', 'b', 'u', 'g', 'g', 'e');
    't' =>
      // await
      return readPrecedingKeyword4(pos - 1, 'a', 'w', 'a', 'i');
    'w' =>
      switch (*(pos - 1)) {
        case 'e':
          // new
          return readPrecedingKeyword1(pos - 2, 'n');
        case 'o':
          // throw
          return readPrecedingKeyword3(pos - 2, 't', 'h', 'r');
        default:
          return false; 
      }
  }
  return false;
}

fn is_parent_keyword (curr_pos: &str) -> bool {
  readPrecedingKeyword5(curPos, 'w', 'h', 'i', 'l', 'e') ||
  readPrecedingKeyword3(curPos, 'f', 'o', 'r') ||
  readPrecedingKeyword2(curPos, 'i', 'f')
}

fn is_punctuator(ch: char) -> bool {
  // 23 possible punctuator endings: !%&()*+,-./:;<=>?[]^{}|~
  ch == '!' ||
  ch == '%' ||
  ch == '&' ||
  (ch > 39 && ch < 48) ||
  (ch > 57 && ch < 64) ||
  ch == '[' || ch == ']' ||
  ch == '^' ||
  ch > 122 && ch < 127
}

fn is_expression_punctuator(ch: char) -> bool {
  // 20 possible expression endings: !%&(*+,-.:;<=>?[^{|~
  return ch == '!' || ch == '%' || ch == '&' ||
    ch > 39 && ch < 47 && ch != 41 || ch > 57 && ch < 64 ||
    ch == '[' || ch == '^' || ch > 122 && ch < 127 && ch != '}';
}

fn is_expression_terminator(curr_pos: &str) -> bool {
  // detects:
  // => ; ) finally catch else class X
  // as all of these followed by a { will indicate a statement brace
  let char_vec: Vec<char> = curr_pos.chars().collect();
  let ch = char_vec[0];

  match ch {
    '>' =>
      return *(curPos - 1) == '=';
    ';' =>
    ')' =>
      return true;
    case 'h':
      return readPrecedingKeyword4(curPos - 1, 'c', 'a', 't', 'c');
    case 'y':
      return readPrecedingKeyword6(curPos - 1, 'f', 'i', 'n', 'a', 'l', 'l');
    case 'e':
      return readPrecedingKeyword3(curPos - 1, 'e', 'l', 's');
  }
  return false;
}



//
//------
//
fn main() {
  println!("Hello, world!");
}
