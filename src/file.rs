use crate::{
    unit::{MM, PX},
    DPI,
};
use image::GenericImageView;
use printpdf::{ColorBits, ColorSpace, Image, ImageTransform, ImageXObject, Mm, PdfDocument, Px};
use ratatui::widgets::ListState;
use std::{
    convert::From,
    error,
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct FileList {
    pub root_dir: PathBuf,
    pub items: Vec<FileItem>,
    pub state: ListState,
}

#[derive(Clone)]
pub struct FileItem {
    pub path: PathBuf,
    pub status: Status,
}

#[derive(Clone)]
pub enum Status {
    Unchecked,
    Checked,
}

impl FileList {
    pub fn load_files(&mut self, root_path: &Path) -> std::io::Result<()> {
        self.items.clear();
        self.root_dir = root_path.to_path_buf();

        let extensions = ["jpg", "jpeg", "png"];

        self.load_files_recursive(root_path, &extensions)?;
        Ok(())
    }

    fn load_files_recursive(&mut self, path: &Path, extensions: &[&str]) -> std::io::Result<()> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.load_files_recursive(&path, extensions)?;
                } else if path.is_file() {
                    if let Some(ext) = path.extension() {
                        match ext.to_str() {
                            Some(ext_str) => {
                                if extensions.contains(&ext_str) {
                                    self.items.push(FileItem {
                                        path,
                                        status: Status::Unchecked,
                                    })
                                }
                            }
                            None => {
                                eprintln!(
                                    "Failed to convert extension to str for file: {:?}",
                                    path
                                );
                            }
                        }
                    }
                }
            }
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                match ext.to_str() {
                    Some(ext_str) => {
                        if extensions.contains(&ext_str) {
                            self.items.push(FileItem {
                                path: path.to_path_buf(),
                                status: Status::Unchecked,
                            });
                        }
                    }
                    None => {
                        eprintln!("Failed to convert extension to str for file: {:?}", path);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn convert_to_pdf(
        &mut self,
        size_width: MM,
        size_height: MM,
        output_name: &str,
    ) -> Result<(), Box<dyn error::Error>> {
        let (doc, first_page, first_layer) =
            PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer");

        let items: Vec<FileItem> = self
            .items
            .iter()
            .filter(|item| matches!(item.status, Status::Checked))
            .cloned()
            .collect();

        let page_width_mm = size_width;
        let page_height_mm = size_height;

        for (index, item) in items.iter().enumerate() {
            if let Status::Unchecked = item.status {
                continue;
            }

            let image = match image::open(item.path.clone()) {
                Ok(image) => image,
                Err(e) => {
                    eprintln!("Failed to open image: {:?} - {}", item.path, e);
                    continue;
                }
            };

            let (width, height) = image.dimensions();

            let current_layer = if index == 0 {
                doc.get_page(first_page).get_layer(first_layer)
            } else {
                let (page_index, layer_index) =
                    doc.add_page(page_width_mm.into(), page_height_mm.into(), "Layer");
                doc.get_page(page_index).get_layer(layer_index)
            };

            let image_data = image.to_rgb8().into_raw();

            let image_file = ImageXObject {
                width: Px(width as usize),
                height: Px(height as usize),
                color_space: ColorSpace::Rgb,
                bits_per_component: ColorBits::Bit8,
                interpolate: false,
                image_data,
                image_filter: None,
                clipping_bbox: None,
                smask: None,
            };
            let image = Image::from(image_file);

            let (img_width, img_height) = (width as f32, height as f32);

            let page_width_px: PX = page_width_mm.into();
            let page_height_px: PX = page_height_mm.into();

            let scale_x = page_width_px.value / img_width;
            let scale_y = page_height_px.value / img_height;
            let scale = scale_x.min(scale_y);

            let translate_x_px = PX::new(0., DPI);
            let translate_y_px = PX::new(
                (page_height_px.value - (img_height * scale)).abs() / 2.,
                DPI,
            );

            let image_transform = ImageTransform {
                translate_x: Some(MM::from(translate_x_px).into()),
                translate_y: Some(MM::from(translate_y_px).into()),
                scale_x: Some(scale),
                scale_y: Some(scale),
                rotate: None,
                dpi: Some(DPI),
            };
            image.add_to_layer(current_layer, image_transform);
        }

        let output_path = self.root_dir.join(format!("{}.pdf", output_name));

        doc.save(&mut BufWriter::new(File::create(output_path)?))?;
        Ok(())
    }

    pub fn convert_sizename_to_size() {}
}
