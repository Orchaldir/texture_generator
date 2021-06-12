use crate::data::EditorData;
use crate::message::EditorMessage;
use crate::tool::tile::TileTool;
use crate::tool::wall::WallTool;
use crate::tool::Tool;
use iced::Element;

pub struct Tools {
    tile_tool: TileTool,
    wall_tool: WallTool,
    current_tool: usize,
}

impl Tools {
    pub fn new() -> Self {
        Tools {
            tile_tool: TileTool::default(),
            wall_tool: WallTool::default(),
            current_tool: 0,
        }
    }

    pub fn get_tool_names(&self) -> Vec<&str> {
        vec![self.tile_tool.get_name(), self.wall_tool.get_name()]
    }

    pub fn get_current_tool(&self) -> usize {
        self.current_tool
    }

    fn get_tool(&self) -> &dyn Tool {
        match self.current_tool {
            1 => &self.wall_tool,
            _ => &self.tile_tool,
        }
    }

    fn get_tool_mut(&mut self) -> &mut dyn Tool {
        match self.current_tool {
            1 => &mut self.wall_tool,
            _ => &mut self.tile_tool,
        }
    }

    pub fn update(&mut self, data: &mut EditorData, message: EditorMessage) -> bool {
        match message {
            EditorMessage::ChangeTool(id) => self.current_tool = id,
            _ => return self.get_tool_mut().update(data, message),
        }

        return false;
    }

    pub fn view_sidebar(&mut self, data: &EditorData) -> Element<EditorMessage> {
        self.get_tool_mut().view_sidebar(data)
    }
}
