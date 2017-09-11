#[derive( Deserialize, Serialize)]
pub struct PaginationResult<T> {
    pub start: bool,
    pub end: bool,
    pub total_count: int,
    pub result: Vec<T>,
}
impl<T> PaginationResult<T> {
    pub fn new() -> PaginationResult {
        PaginationResult
    }
    pub fn start(mut self, start: bool) -> Self {
        self.start = start;
        self
    }
    pub fn end(mut self, end: bool) -> Self {
        self.end = end;
        self
    }
    pub fn total_count(mut self, total_count: int) -> Self {
        self.total_count = total_count;
        self
    }
    pub fn result<T>(mut self, result: Vec<T>) -> Self {
        self.result = result;
        self
    }
}
