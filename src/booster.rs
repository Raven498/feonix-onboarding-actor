use tokio::sync::{mpsc, oneshot};

use crate::*;

// ##################################################### //
// ################### ACTOR BACKEND ################### //
// ##################################################### //

struct Booster {
   receiver: mpsc::Receiver<BoosterMessage>,
   admin: Option<AdminHandle>,
}

#[derive(Debug)]
enum BoosterMessage {
    BoostGrade{},
    SetAdmin{admin_handle: AdminHandle},
}

impl Booster {
    fn new(receiver: mpsc::Receiver<BoosterMessage>) -> Self {
        Booster{
            receiver: receiver,
            admin: None,
        }
    }

    async fn handle_message(&mut self, msg: BoosterMessage) {
        println!(
            "[Actor] Booster is running handle_message() with new BoosterMessage: {:?}",
            msg
        );
        match msg {
            BoosterMessage::BoostGrade {} => {
                println!("[ACTOR]: Booster boosting all grades retrieved from Admin!");
                if let Some(ad) = &self.admin{
                    let grades : Vec<f64> = ad.get_all_student_grades().await;
                    let mut index = 0;
                    let mut newGrades = grades.clone();
                    loop{
                        if index == newGrades.len(){
                            break;
                        }
                        newGrades[index] = 100.0;
                        index = index + 1;
                    }
                    ad.submit_student_grades(newGrades).await;
                    let updatedGrades : Vec<f64> = ad.get_all_student_grades().await;
                    println!("[ACTOR]: Booster sees: {:?}", updatedGrades);
                } else {
                    println!("[ACTOR]: Admin not initialized so Booster didn't do anything");
                }
                
            },
            BoosterMessage::SetAdmin{admin_handle} => {
                println!("[ACTOR]: Booster setting Admin");
                self.admin = Some(admin_handle);
            },
        };
    }
}

// ###################################################### //
// ################### ACTOR FRONTEND ################### //
// ###################################################### //

async fn run_booster_actor(mut actor: Booster) {
    // TODO
    while let Some(msg) = actor.receiver.recv().await{
        println!("[run_booster_actor] is blocking until a BoosterMessage is received");
        actor.handle_message(msg).await;
    }
}

#[derive(Clone, Debug)]
pub struct BoosterHandle {
    sender: mpsc::Sender<BoosterMessage>,
}

impl BoosterHandle {
    pub async fn new() -> Self {
        // TODO
        let (sender, receiver) = mpsc::channel(8);
        let actor: Booster = Booster::new(receiver);
        tokio::spawn(run_booster_actor(actor));
        BoosterHandle {sender: sender}
    }

    pub async fn boost_grades(&self){
        let msg: BoosterMessage = BoosterMessage::BoostGrade{ 
        };
        let _ = self.sender.send(msg).await;
    }

    pub async fn set_admin(&self, admin_handle : AdminHandle){
        let msg: BoosterMessage = BoosterMessage::SetAdmin{
            admin_handle: admin_handle,
        };
        let _ = self.sender.send(msg).await;
    }
}
