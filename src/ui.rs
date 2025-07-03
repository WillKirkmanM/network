use crate::network_info::NetworkData;
use eframe::egui::{self, Color32, RichText, Ui};

pub fn render(ctx: &egui::Context, data: &NetworkData) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Network Information Viewer");
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            render_interfaces_section(ui, data);
            ui.add_space(20.0);
            render_dns_section(ui, data);
        });
    });
}

fn render_interfaces_section(ui: &mut Ui, data: &NetworkData) {
    ui.label(RichText::new("Network Interfaces").size(18.0).color(Color32::LIGHT_BLUE));
    ui.add_space(5.0);
    
    egui::Grid::new("interfaces_grid")
        .num_columns(4)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label(RichText::new("Name").strong());
            ui.label(RichText::new("IP Address").strong());
            ui.label(RichText::new("Received").strong());
            ui.label(RichText::new("Transmitted").strong());
            ui.end_row();

            for iface in &data.interfaces {
                ui.label(&iface.name);
                ui.label(iface.ip_addr.to_string());
                ui.label(format!("{:.2} MB", iface.received as f64 / 1_048_576.0));
                ui.label(format!("{:.2} MB", iface.transmitted as f64 / 1_048_576.0));
                ui.end_row();
            }
        });
}

fn render_dns_section(ui: &mut Ui, data: &NetworkData) {
    ui.label(RichText::new("DNS Information").size(18.0).color(Color32::LIGHT_BLUE));
    ui.add_space(5.0);

    if let Some(dns) = &data.dns_info {
        egui::Grid::new("dns_grid")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .show(ui, |ui| {
                ui.label(RichText::new("Hostname").strong());
                ui.label(&dns.hostname);
                ui.end_row();

                ui.label(RichText::new("Host IPs").strong());
                ui.vertical(|ui| {
                    for ip in &dns.ips {
                        ui.label(ip.to_string());
                    }
                });
                ui.end_row();
            });
    } else {
        ui.label("Could not retrieve DNS information.");
    }
}