use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::door::DoorTool;
use crate::tool::furniture::FurnitureTool;
use crate::tool::map::file::FileTool;
use crate::tool::map::new::NewMapTool;
use crate::tool::tile::TileTool;
use crate::tool::wall::WallTool;
use crate::tool::window::WindowTool;
use crate::tool::Tool;
use iced::Element;

pub struct Tools {
    tools: Vec<Box<dyn Tool>>,
    current_tool: usize,
}

impl Tools {
    pub fn new() -> Self {
        Tools {
            tools: vec![
                Box::new(TileTool::default()),
                Box::new(WallTool::default()),
                Box::new(DoorTool::default()),
                Box::new(WindowTool::default()),
                Box::new(FurnitureTool::new(1, 1, 10)),
                Box::new(NewMapTool::new(10, 10, 20)),
                Box::new(FileTool::default()),
            ],
            current_tool: 0,
        }
    }

    pub fn get_tool_names(&self) -> Vec<&str> {
        self.tools.iter().map(|t| t.get_name()).collect()
    }

    pub fn get_current_tool(&self) -> usize {
        self.current_tool
    }

    pub fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeTool(id) => self.current_tool = id,
            _ => return self.tools[self.current_tool].update(data, message),
        }

        false
    }

    pub fn view_sidebar(&mut self, data: &EditorData) -> Element<EditorMessage> {
        self.tools[self.current_tool]
            .view_sidebar(data)
            .max_width(250)
            .padding(10)
            .spacing(10)
            .into()
    }
}
