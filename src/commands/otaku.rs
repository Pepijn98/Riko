use serenity::utils::Colour;
use kitsu::KitsuReqwestRequester;
use reqwest::Client;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{Args, CommandResult, macros::command};

#[command]
fn anime(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	if args.is_empty() {
		let _ = msg.channel_id.say(&ctx.http, "Which anime should I search for?");
		return Ok(());
	}

	let search = args.rest();
	let client = Client::new();
	if let Ok(result) = client.search_anime(|f| f.filter("text", &search)) {
		if let Some(ani) = result.data.get(0) {
			let anime_title = &ani.attributes.canonical_title;
			let anime_synopsis = &ani.attributes.synopsis;
			let anime_age_rating = match ani.attributes.age_rating {
				Some(ref x) => format!("{:?}", x),
				None => "-".to_owned(),
			};
			let anime_average_rating = match ani.attributes.average_rating {
				Some(ref x) => x.to_string(),
				None => "-".to_owned(),
			};
			let anime_type = match ani.attributes.kind.name(){
				Ok(x) => x,
				Err(_) => "-".to_owned(),
			};
			let anime_airing_status = ani.attributes.airing_status();
			let anime_airing_status_name = anime_airing_status.name();
			let anime_episode_count = match ani.attributes.episode_count {
				Some(ref x) => x.to_string(),
				None => "-".to_owned(),
			};
			let anime_start_date = &ani.attributes.start_date;
			let anime_end_date = match ani.attributes.end_date {
				Some(ref x) => x.to_owned(),
				None => "?".to_owned(),
			};

			let anime_poster_image = match ani.attributes.poster_image.largest(){
				Some(x) => x.to_owned(),
				None => "".to_owned(),
			};

			let _ = match msg.channel_id.send_message(&ctx.http, |cm| cm
				.embed(|ce| ce
					.title(&anime_title)
					.url(&ani.url())
					.color(Colour::from_rgb(246, 219, 216))
					.description(&anime_synopsis)
					.thumbnail(anime_poster_image)
					.field("Average Rating", &anime_average_rating, true)
					.field("Type", &anime_type, true)
					.field("Age Rating", &anime_age_rating, true)
					.field("Episodes", &anime_episode_count, true)
					.field("Status", anime_airing_status_name, true)
					.field("Start/End", &format!("{:?} until {}", anime_start_date, &anime_end_date), true)
				)
			){
				Ok(msg) => msg,
				Err(why) => {
					let _ = msg.channel_id.say(&ctx.http, format!("Error sending embed:\n{:?}", why));
					return Ok(());
				},
			};

            Ok(())
		} else {
			let _ = msg.channel_id.say(&ctx.http, "Failed to get anime info.");

            Ok(())
		}
	} else {
		let _ = msg.channel_id.say(&ctx.http, "Failed to get anime info.");

        Ok(())
	}
}

#[command]
fn manga(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
	if args.is_empty() {
		let _ = msg.channel_id.say(&ctx.http, "Which manga should I search for?");
		return Ok(());
	}

	let search = args.rest();
	let client = Client::new();
	if let Ok(result) = client.search_manga(|f| f.filter("text", &search)){
		if let Some(mango) = result.data.get(0) {
			let manga_title = &mango.attributes.canonical_title;
			let mut manga_synopsis = mango.attributes.synopsis.to_owned();
			if &manga_synopsis == "" {
				manga_synopsis = "-".to_owned();
			}
			let manga_type = &mango.attributes.kind;
			let manga_average_rating = match mango.attributes.average_rating {
				Some(ref x) => x.to_string(),
				None => "-".to_owned(),
			};
			let manga_volume_count = match mango.attributes.volume_count {
				Some(ref x) => x.to_string(),
				None => "-".to_owned(),
			};
			let manga_chapter_count = match mango.attributes.chapter_count{
				Some(ref x) => x.to_string(),
				None => "-".to_owned(),
			};
			let manga_start_date = match mango.attributes.start_date {
				Some(ref x) => x.to_owned(),
				None => "?".to_owned(),
			};
			let manga_end_date = match mango.attributes.end_date {
				Some(ref x) => x.to_owned(),
				None => "?".to_owned(),
			};
			let manga_poster_image = match mango.attributes.poster_image.largest() {
				Some(ref x) => x.to_owned(),
				None => "",
			};

			let _ = match msg.channel_id.send_message(&ctx.http, |cm| cm
				.embed(|ce| ce
					.title(&manga_title)
					.url(&mango.url())
					.colour(Colour::from_rgb(246, 219, 216))
					.description(&manga_synopsis)
					.thumbnail(manga_poster_image)
					.field("Average Rating", &manga_average_rating, true)
					.field("Type", &format!("{:?}", manga_type), true)
					.field("Volumes", &manga_volume_count, true)
					.field("Chapters", &manga_chapter_count, true)
					.field("Start/End", &format!("{} until {}", &manga_start_date, &manga_end_date), true)
				)
			){
				Ok(msg) => msg,
				Err(why) => {
					let _ = msg.channel_id.say(&ctx.http, format!("Error sending embed:\n{:?}", why));
					return Ok(());
				},
			};

            Ok(())
		} else {
			let _ = msg.channel_id.say(&ctx.http, "Failed to get manga info.");

            Ok(())
		}
	} else {
		let _ = msg.channel_id.say(&ctx.http, "Failed to get manga info.");

        Ok(())
	}
}