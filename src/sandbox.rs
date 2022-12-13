struct Item {}
impl Item {
    fn change(&mut self) {}
}

struct Player {}
impl Player {
    fn play(&self) {}
}

struct Sandbox {
    item: Item,
    player: Player,
}
impl Sandbox {
    pub fn start(&mut self) {
        let mut player = &self.player;

        while true {
            player.play();

            self.item.change();

            if true {
                player = self.get_next_player();
            }
        }
    }

    #[inline(always)]
    fn get_next_player(&self) -> &Player {
        &self.player
    }
}
