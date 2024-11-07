use crate::order::Order;
use escpos::driver::UsbDriver;
use escpos::printer::Printer;
use escpos::utils::{
    JustifyMode, Protocol, QRCodeCorrectionLevel, QRCodeModel, QRCodeOption, UnderlineMode,
};

pub struct ReceiptPrinter {
    printer: Printer<UsbDriver>,
}

impl ReceiptPrinter {
    pub fn new(vendor_id: u16, product_id: u16) -> anyhow::Result<Self> {
        let driver = UsbDriver::open(vendor_id, product_id, None)?;
        let printer = Printer::new(driver, Protocol::default(), None);

        Ok(Self { printer })
    }

    pub fn print_order(&mut self, order: Order) -> anyhow::Result<()> {
        self.printer.init()?
            .justify(JustifyMode::CENTER)?
            .bit_image("htl_logo.pbm")?
            .feeds(2)?
            .size(4,4)?
            .double_strike(true)?
            .writeln(&format!("{:04}", order.id))?
            .double_strike(false)?
            .feeds(2)?
            .reset_size()?
            .writeln("Vielen Dank, dass Sie bei bei uns bestellt haben :) Im folgenden Segment sehen Sie alle Produkte, welche Sie bestellt haben.  ")?
            .feed()?
            // TODO SHOW ALL PRODUCTS
            .writeln(&format!("{:>5}  {:<24}  {:>10}", "Menge", "Bezeichnung", "Preis"))?;

        for entry in order.entries {
            self.printer.writeln(&format!(
                "{:>5}  {:<24}  {:>3} Punkte",
                entry.amount, entry.product.name, entry.amount*entry.product.price
            ))?;
        }

        self.printer.writeln(&format!("{}", "-".repeat(43)))?
            .double_strike(true)?
            .writeln(&format!("{:>43}", "200 Punkte"))?
            .double_strike(false)?
            .feed()?
            .writeln("Um Ihre Produkte abzuholen, warten Sie, bis auf dem Bildschirm angezeigt wird, dass Ihre Bestellung fertig ist. Dannach kommen Sie zum schalter und wir geben Ihnen die Produkte.")?
            .feed()?
            .writeln("Informatik HTL St. Poelten")?
            .qrcode_option("https://www.htlstp.ac.at/abteilungen/informatik", QRCodeOption::new(QRCodeModel::Model1, 6, QRCodeCorrectionLevel::L))?
            .feeds(4)?
            .print_cut()?;
        Ok(())
    }
}
