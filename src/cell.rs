#[derive(Debug, Clone, PartialEq)]
pub enum CellContent {
    Empty,
    Queen,
    Attack(u8),
}

#[derive(Debug, Clone)]
pub struct Cell {
    x: usize,
    y: usize,
    i: usize,
    content: CellContent,
}

impl Cell {
    pub fn new(x: usize, y: usize, i: usize) -> Self {
        Cell {
            x,
            y,
            i,
            content: CellContent::Empty,
        }
    }

    pub fn get_xyi(&self) -> (&usize, &usize, &usize) {
        (&self.x, &self.y, &self.i)
    }

    pub fn get_content(&self) -> &CellContent {
        &self.content
    }

    pub fn is_queen(&self) -> bool {
        match &self.content {
            CellContent::Queen => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.content {
            CellContent::Empty => true,
            _ => false,
        }
    }

    pub fn is_attacked(&self) -> bool {
        match &self.content {
            CellContent::Attack(_) => true,
            _ => false,
        }
    }

    pub fn toggle(&mut self) -> Result<&CellContent, String> {
        match &self.content {
            CellContent::Empty => {
                self.content = CellContent::Queen;
                Ok(&self.content)
            }
            CellContent::Queen => {
                self.content = CellContent::Empty;
                Ok(&self.content)
            }
            _ => Err("Attacked cell not avaliable".to_string()),
        }
    }

    pub fn attack(&mut self) -> Result<(), String> {
        match &self.content {
            CellContent::Empty => {
                self.content = CellContent::Attack(1);
                Ok(())
            }
            CellContent::Attack(a) => {
                self.content = CellContent::Attack(a + 1);
                Ok(())
            }
            _ => Err("Cell not available for attack".to_string()),
        }
    }

    pub fn relieve(&mut self) -> Result<(), String> {
        match &self.content {
            CellContent::Attack(a) if a == &1_u8 => {
                self.content = CellContent::Empty;
                Ok(())
            }
            CellContent::Attack(a) => {
                self.content = CellContent::Attack(a - 1);
                Ok(())
            }
            _ => Err("Cell not available for relieve".to_string()),
        }
    }

    pub fn attack_or_relieve(&mut self, attack: &bool) -> Result<(), String> {
        if *attack {
            self.attack()
        } else {
            self.relieve()
        }
    }
}
