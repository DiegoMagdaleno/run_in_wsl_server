use std::collections::HashMap;
extern crate freedesktop_entry_parser;
use crate::lib::models::Application;

pub fn parse_entries(entry_location_map: HashMap<&str, Vec<String>>) -> Option<Vec<Application>> {
    let global_vector = entry_location_map.get("global");
    let mut paths_of_global_services = Vec::new();
    let mut desktop_parsed_entries = Vec::new();
    if let Some(entries) = global_vector {
        for gentry in entries {
            let formatted_entry = format!("/usr/share/applications/{}", gentry);
            paths_of_global_services.push(formatted_entry);
        }
    }

    for gunparsed in &paths_of_global_services {
        let parse_freedesktop = freedesktop_entry_parser::parse_entry(gunparsed).ok()?;
        let desktop_entry_sec = parse_freedesktop.section("Desktop Entry");
        let (mut freedesk_desc, mut freedesk_exec, mut freedesk_name) = ("", "", "");

        if let Some(has_name) = desktop_entry_sec.attr("Name") {
            freedesk_name = has_name;
        }

        if let Some(has_exec) = desktop_entry_sec.attr("Exec") {
            freedesk_exec = has_exec;
        }

        if let Some(has_comment) = desktop_entry_sec.attr("Comment") {
            freedesk_desc = has_comment;
        }


        let desktop_entry = Application::new(freedesk_name, freedesk_exec, freedesk_desc);
        desktop_parsed_entries.push(desktop_entry);
    }

    if desktop_parsed_entries.len() > 0 {
        Some(desktop_parsed_entries)
    } else {
        None
    }
}
