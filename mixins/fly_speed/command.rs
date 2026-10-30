// /flyspeed says the speed, /flyspeed <number> sets it, +1 and -0.5 step it,
// and /flyspeed reset goes back to the game's own. Every other command is
// left alone.
fn fly_speed_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::fly_speed as m;
    let Some(args) = command.strip_prefix("flyspeed") else {
        return;
    };
    if !game.native_on(m::ID) || !(args.is_empty() || args.starts_with(' ')) {
        return;
    }
    *ran = true;
    let current = m::multiplier(|| game.native_setting(m::ID, "multiplier"));
    match args.trim() {
        "" => {
            game.native_chat(&format!("Fly speed: {}", m::label(current)));
            game.native_chat("Use /flyspeed <number>, /flyspeed +1 or /flyspeed reset");
        }
        "reset" | "default" => {
            m::set_multiplier(1.0);
            game.native_set_setting(m::ID, "multiplier", &m::stored(1.0));
            game.native_chat("Fly speed: 1x, the game's own");
        }
        other => match m::parse(other, current) {
            Ok(multiplier) => {
                m::set_multiplier(multiplier);
                game.native_set_setting(m::ID, "multiplier", &m::stored(multiplier));
                game.native_chat(&format!("Fly speed: {}", m::label(multiplier)));
            }
            Err(said) => game.native_chat(&said),
        },
    }
}
