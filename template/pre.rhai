// Binary name defaults to the tool name, always lowercase.
let bin = variable::get("bin_name");
if bin == "" { bin = variable::get("project-name"); }
variable::set("bin_name", bin.to_lower());
