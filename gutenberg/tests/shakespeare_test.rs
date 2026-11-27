use gutenberg::shakespeare;

#[test]
fn test_read_play() {
    let reader = shakespeare::new_reader();
    let play = reader.read_play("ALL’S WELL THAT ENDS WELL");
    
    assert_eq!(play.title, "ALL’S WELL THAT ENDS WELL");
    assert!(!play.acts.is_empty());
    
    let first_act = &play.acts[0];
    assert!(first_act.number.contains("ACT I"));
    assert!(!first_act.scenes.is_empty());
    
    println!("Play: {}", play.title);
    println!("Acts: {}", play.acts.len());
    for act in &play.acts {
        println!("  {}: {} scenes", act.number, act.scenes.len());
    }
    
    let first_scene = &play.acts[0].scenes[0];
    println!("First scene dialogues: {}", first_scene.dialogues.len());
    for i in 0..5.min(first_scene.dialogues.len()) {
        println!("    {:#?}", first_scene.dialogues[i]);
    }
}
