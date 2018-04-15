mod chars;

use self::chars::*;
use std::cmp::max;
use std::fmt;
use std::collections::HashMap;

#[allow(non_camel_case_types, dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum States {

    str_double_quotes_end   ,
    str_simple_quotes_end   ,
    str_double_quotes       ,
    str_simple_quotes       ,
    hexa                    ,
    zero                    ,
    exponent                ,
    group_ch                ,
    char_before_eq          ,
    char_escaped_simple     ,
    char_escaped_double     ,
    escaping_from_simple    ,
    escaping_from_double    ,
    float                   ,
    negative                ,
    comment                 , //irelevant
    number                  ,
    number_after_exp        ,
    del_character           ,
    newl                    ,
    c_r                     ,
    space                   ,
    ident                   ,
    initial                 ,
    end                     ,
    err                     

}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum ErrorStates {
    cannot_begin_with       ,
    unexpected_newl         ,
    expected_newl           ,
    unknown
}


impl States {
    fn max(v: Vec<States>) -> States {
        v.into_iter().fold(States::err, |m, x|  {
            if x < m {
                x
            } else {
                m
            }
        })
    }
}

impl States{
    fn next(&self, ch: char) -> Result<States, ErrorStates> {
        let c = CharIdentifier::new(ch);
        match self {
            &States::initial => {
                if c.is_neg() {
                    return Ok(States::negative)
                } 
                if c.is_zero() {
                    return Ok(States::zero)
                }
                if c.is_c_r() {
                    return Ok(States::c_r)
                }
                if c.is_first_id_char() {
                    return Ok(States::ident)
                }
                if c.is_space() {
                    return Ok(States::space)
                }
                if c.is_newline() {
                    return Ok(States::newl)
                }
                if c.is_digit() {
                    return Ok(States::number)
                }
                if c.is_before_eq() {
                    return Ok(States::char_before_eq)
                }
                if c.is_del_character() {
                    return Ok(States::del_character)
                }
                if c.is_hash() {
                    return Ok(States::comment)
                }
                if c.is_simple_quote() {
                    return Ok(States::str_simple_quotes)
                }
                if c.is_double_quote() {
                    return Ok(States::str_double_quotes)
                }
                return Err(ErrorStates::cannot_begin_with)
            },
            &States::ident => {
                if c.is_id_char() {
                    return Ok(States::ident)
                }
                return Ok(States::end)
                
            },
            &States::space => {
                if c.is_space() {
                    return Ok(States::space)
                }
                return Ok(States::end)
            },
            &States::newl => {
                if c.is_tab() {
                    return Ok(States::newl)
                }
                return Ok(States::end)
            },
            &States::c_r => {
                if c.is_newline() {
                    return Ok(States::newl)
                }
                return Err(ErrorStates::expected_newl)
            },
            &States::del_character => {
                return Ok(States::end)
            },
            &States::number => {
                if c.is_digit() {
                    return Ok(States::number)
                }
                if c.is_point() {
                    return Ok(States::float)
                }
                if c.is_e() {
                    return Ok(States::exponent)
                }
                return Ok(States::end)
            },
            &States::number_after_exp => {
                if c.is_digit() {
                    return Ok(States::number)
                }
                return Ok(States::end)
            },
            &States::float => {
                if c.is_digit() {
                    return Ok(States::number)
                }
                return Ok(States::end)
            },
            &States::comment => {
                if c.is_newline() {
                    return Ok(States::end)
                }
                return Ok(States::comment)
            },
            &States::str_simple_quotes => {
                if c.is_escape() {
                    return Ok(States::escaping_from_simple)
                }
                if c.is_simple_quote() {
                    return Ok(States::str_simple_quotes_end)
                }
                if c.is_newline() {
                    return Err(ErrorStates::unexpected_newl)
                }
                return Ok(States::str_simple_quotes)
                
            },
            &States::str_simple_quotes_end => {
                return Ok(States::end)
            },
            &States::str_double_quotes => {
                if c.is_escape() {
                    return Ok(States::escaping_from_double)
                }
                if c.is_double_quote() {
                    return Ok(States::str_double_quotes_end)
                }
                // Remove this if you want to have newlines in double quotes
                //if c.is_newline() {
                //    return Err(ErrorStates::unexpected_newl)
                //}
                return Ok(States::str_double_quotes)
                
            },
            &States::str_double_quotes_end => {
                return Ok(States::end)
            },
            &States::negative => {
                if c.is_digit() {
                    return Ok(States::number)
                }
                return Ok(States::end)
            },
            &States::escaping_from_simple => {
                if c.is_escape() {
                    return Ok(States::str_simple_quotes)
                }
                if c.is_simple_quote() {
                    return Ok(States::str_simple_quotes)
                }
                return Ok(States::char_escaped_simple)
            },
            &States::escaping_from_double => {
                if c.is_escape() {
                    return Ok(States::str_double_quotes)
                }
                if c.is_double_quote() {
                    return Ok(States::str_double_quotes)
                }
                return Ok(States::char_escaped_double)
            },
            &States::char_escaped_simple => {
                return Ok(States::str_simple_quotes)
            },
            &States::char_escaped_double => {
                return Ok(States::str_double_quotes)
            },
            &States::char_before_eq => {
                if c.is_equal() {
                    return Ok(States::group_ch)
                }
                return Ok(States::end)
            },
            &States::group_ch => {
                return Ok(States::end)
            },
            &States::exponent => {
                if c.is_neg() {
                    return Ok(States::negative)
                }
                if c.is_plus() {
                    return Ok(States::exponent)
                }
                if c.is_digit() {
                    return Ok(States::number_after_exp)
                }
                return Ok(States::end)
            },
            &States::zero => {
                if c.is_x() {
                    return Ok(States::hexa)
                }
                if c.is_digit() {
                    return Ok(States::number)
                }
                return Ok(States::end)
            },
            &States::hexa => {
                if c.is_hexa() {
                    return Ok(States::hexa)
                }
                return Ok(States::end)
            },
            _            => Err(ErrorStates::unknown)
        }
    }
}

struct Dfa {
    pos: usize,
    state: States,
    input: Vec<char>,
    had_error: bool
}


impl Dfa {
    fn new(input: String) -> Dfa {
        let chars_v = input.chars().collect::<Vec<char>>();
        Dfa { pos: 0, state: States::initial, input: chars_v, had_error: false }
    }

    fn reset(&mut self) {
        self.state = States::initial;
    }

    fn parse_one(&mut self) -> Result<char, String> {
        if self.pos == self.input.len() {
            self.state = States::end;
            return Err(String::from("eof"))
        }

        let ch = self.input[self.pos];


        self.state = match self.state.next(ch) {
            Ok(s) => s,
            Err(er) => {
                return Err(format!("{:?}", er))
            }
        };

        if self.state != States::end {
            self.pos += 1;
        } else {
            return Err(String::from("eot"));
        }

        Ok(ch)
    }

    fn get_token(&mut self) -> Option<(String, Vec<States>, usize)> {
        let mut out = String::new();
        let mut s_v = Vec::new();
        
        if self.had_error {
            return None
        }
        while self.state != States::end && self.state != States::err {
            s_v.push(self.state.clone());
            let pos = self.pos;
            let mut err = false;
            let mut msg = String::new();
            match self.parse_one() {
                Ok(ch) => out.push(ch),
                Err(message) => {
                    match message.as_ref() {
                        "eot" => break,
                        "eof" => return None,
                        m     => {
                            err = true;
                            msg = String::from(m);
                        }
                    }
                }
            }

            if err {
                self.had_error = true;
                return Some((msg, vec![States::err], pos))
            }
        }

        return Some((out, s_v, self.pos-1))

    }

}


pub struct Tokenizer {
    dfa: Dfa,
    table: HashMap<(String, String), Vec<usize>>,
}


impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        let dfa = Dfa::new(input);
        Tokenizer { dfa, table: HashMap::new() }
    }

    pub fn get_table(self) -> HashMap<(String, String), Vec<usize>>{
        self.table
    }

    pub fn get_token(&mut self) -> Option<(String, String, usize)> {
        self.dfa.reset();
        if let Some((token_val, visited_states, chars_cons)) = self.dfa.get_token() {
            let dominant_state = States::max(visited_states);

            if dominant_state == States::initial {
                return None;
            }

            let m = match dominant_state{
                States::char_before_eq        => String::from("operator"),
                States::str_double_quotes_end => String::from("double_string"),
                States::str_simple_quotes_end => String::from("simple_string"),
                States::del_character         => String::from("operator"),
                States::group_ch              => String::from("group_operator"),
                s                             => format!("{:?}", s)

            };

            let key = (m, token_val);

            if let None = self.table.get_mut(&key) {
                self.table.insert(key.clone(), Vec::new());
            }
            
            self.table.get_mut(&key).unwrap().push(chars_cons + 1 - key.1.len());

            return Some((key.1, key.0, chars_cons))

        } else {
            None
        }

    }
}