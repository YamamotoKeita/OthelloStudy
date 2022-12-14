struct SPlayer {}
impl SPlayer {
    fn play(&self, sandbox: &Sandbox) {}
}

struct Sandbox {
    player: SPlayer,
}
impl Sandbox {
    pub fn start(&mut self) {
        let mut player = &self.player;
        self.change();

        //if true {
        player = self.get_player();
        //}
        player.play(&self);
    }

    fn change(&mut self) {}

    fn get_player(&self) -> &SPlayer {
        &self.player
    }
}
