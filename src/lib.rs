use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro_derive(ToJson)]
pub fn derive_to_json(item: TokenStream) -> TokenStream {
    let struct_string = item.to_string();
    let mut struct_head = struct_string.split("{").collect::<Vec<&str>>()[0]
        .split(" ")
        .collect::<Vec<&str>>();
    if struct_head.last().unwrap().is_empty() {
        struct_head.remove(struct_head.len() - 1);
    }
    let struct_name = struct_head.last().unwrap().replace("\n", "");

    let struct_fields_string = struct_string.split("{").collect::<Vec<&str>>()[1]
        .replace("}", "");
    let struct_fields = struct_fields_string.split(",").collect::<Vec<&str>>().iter().map(|x| {
        let vals = x.split(":").collect::<Vec<&str>>();
        let field_details = vals[0].trim().split(" ").last().unwrap().to_string();
        let struct_type = vals[1].trim().to_string();
        return (field_details, struct_type)
    }).collect::<Vec<(String, String)>>();

    let quote = "\\\"";

    let mut output = String::new();
    output += "impl ToJson for ";
    output += &struct_name;
    output += " {\n";
    output += "fn to_json(&self) -> String {\n";
    output += "let mut output = String::new();";
    output += "output += \"{\";";
    for f in struct_fields {
        output += "output += \"";
        output += quote;
        output += &f.0;
        output += quote;
        output += "\"";
        output += ";\n";
        output += "output += \":\";";
        output += "output += &self.";
        output += &f.0;
        output += ".to_json()";
        output += ";\n";
        output += "output += \",\";";
    }
    output += "output = output[0..output.chars().count() - 1].to_string();\n";
    output += "output += \"}\";";
    output += "return output;\n}";
    output += "\n}";

    TokenStream::from_str(&output).unwrap()
}
