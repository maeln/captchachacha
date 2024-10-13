#![windows_subsystem = "windows"]

use std::{cell::RefCell, rc::Rc};

use fltk::{
    app::{self},
    button::Button,
    frame::Frame,
    group,
    image::{self},
    prelude::*,
    window::Window,
};

static IMG1: &[u8; 182554] = include_bytes!("../assets/1.JPG");
static IMG2: &[u8; 247447] = include_bytes!("../assets/2.JPG");
static IMG3: &[u8; 123839] = include_bytes!("../assets/3.JPG");
static IMG4: &[u8; 143067] = include_bytes!("../assets/4.JPG");
static IMG5: &[u8; 96471] = include_bytes!("../assets/5.JPG");
static IMG6: &[u8; 118124] = include_bytes!("../assets/6.JPG");
static IMG7: &[u8; 126857] = include_bytes!("../assets/7.JPG");

fn final_eval(hp: Rc<RefCell<i32>>, fr1: Rc<RefCell<Frame>>, fr2: Rc<RefCell<Frame>>) {
    let hpb = hp.borrow();
    let mut fr1b = fr1.borrow_mut();
    let mut fr2b = fr2.borrow_mut();
    if *hpb >= 5 {
        fr1b.set_label("SUCCESS");
        fr2b.set_label("Sufficient level of existencial dread.")
    } else {
        fr1b.set_label("FAILED");
        fr2b.set_label("Clear lack of existencial dread detected. You are a robot.")
    }
}

fn main() {
    let human_point: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));

    let app = app::App::default();

    let i1 = image::JpegImage::from_data(IMG1).unwrap();
    let i2 = image::JpegImage::from_data(IMG2).unwrap();
    let i3 = image::JpegImage::from_data(IMG3).unwrap();
    let i4 = image::JpegImage::from_data(IMG4).unwrap();
    let i5 = image::JpegImage::from_data(IMG5).unwrap();
    let i6 = image::JpegImage::from_data(IMG6).unwrap();
    let i7 = image::JpegImage::from_data(IMG7).unwrap();

    // TO ANY WHO MIGHT LAY THEIR EYES UPON THEE
    // FORGIVE ME FOR THE SIN YOU ARE ABOUT TO SEE

    let mut window = Window::default()
        .with_size(700, 600)
        .center_screen()
        .with_label("existancha");

    let wizard = group::Wizard::default_fill();

    let intro = group::Group::default_fill();
    let fl = group::Flex::default_fill().column();
    let capttext =
        Frame::default().with_label("Please answer a few question to verify that you are human.");
    let mut start_btn = Button::default().with_label("start");
    fl.end();
    intro.end();

    let step1 = group::Group::default_fill();
    let mut flex1 = group::Flex::default_fill().column();
    let mut flex2 = group::Flex::default().column();
    let mut image1 = Frame::default().with_size(600, 400);
    image1.set_image(Some(i1.clone()));
    let question1 = Frame::default()
        .with_size(80, 30)
        .with_label("Does this picture contain a cat ?");
    flex2.fixed(&question1, 40);
    flex2.end();
    let flex3 = group::Flex::default().row();
    let mut yes1_b = Button::default().with_label("yes");
    let mut no1_b = Button::default().with_label("no");
    flex3.end();
    flex1.fixed(&flex3, 40);
    flex1.end();
    step1.end();

    let step2 = group::Group::default_fill();
    let mut flex21 = group::Flex::default_fill().column();
    let mut flex22 = group::Flex::default().column();
    let mut image2 = Frame::default().with_size(600, 400);
    image2.set_image(Some(i2.clone()));
    let question2 = Frame::default()
        .with_size(80, 30)
        .with_label("Does this picture contain flowers ?");
    flex22.fixed(&question2, 40);
    flex22.end();
    let flex23 = group::Flex::default().row();
    let mut yes2_b = Button::default().with_label("yes");
    let mut no2_b = Button::default().with_label("no");
    flex23.end();
    flex21.fixed(&flex23, 40);
    flex21.end();
    step2.end();

    let step3 = group::Group::default_fill();
    let mut flex31 = group::Flex::default_fill().column();
    let mut flex32 = group::Flex::default().column();
    let mut image3 = Frame::default().with_size(600, 400);
    image3.set_image(Some(i3.clone()));
    let question3 = Frame::default()
        .with_size(80, 30)
        .with_label("Does this picture contain a cat ?");
    flex32.fixed(&question3, 40);
    flex32.end();
    let flex33 = group::Flex::default().row();
    let mut yes3_b = Button::default().with_label("yes");
    let mut no3_b = Button::default().with_label("no");
    flex33.end();
    flex31.fixed(&flex33, 40);
    flex31.end();
    step3.end();

    let step4 = group::Group::default_fill();
    let mut flex41 = group::Flex::default_fill().column();
    let mut flex42 = group::Flex::default().column();
    let mut image4 = Frame::default().with_size(600, 400);
    image4.set_image(Some(i3.clone()));
    let question4 = Frame::default()
        .with_size(80, 30)
        .with_label("How often do you think about death ?");
    flex42.fixed(&question4, 40);
    flex42.end();
    let flex43 = group::Flex::default().row();
    let mut never4_b = Button::default().with_label("never");
    let mut sometimes4_b = Button::default().with_label("sometimes");
    let mut often4_b = Button::default().with_label("often");
    let mut dontknow4_b = Button::default().with_label("I don't know");
    flex43.end();
    flex41.fixed(&flex43, 40);
    flex41.end();
    step4.end();

    let step5 = group::Group::default_fill();
    let mut flex51 = group::Flex::default_fill().column();
    let mut flex52 = group::Flex::default().column();
    let mut image5 = Frame::default().with_size(600, 400);
    image5.set_image(Some(i4.clone()));
    let question5 = Frame::default()
        .with_size(80, 30)
        .with_label("Do you feel like a passenger of your own existence ?");
    flex52.fixed(&question5, 40);
    flex52.end();
    let flex53 = group::Flex::default().row();
    let mut no5_b = Button::default().with_label("No");
    let mut yes5_b = Button::default().with_label("Yes");
    let mut deftones5_b = Button::default().with_label("Just like always, still your passenger");
    flex53.end();
    flex51.fixed(&flex53, 40);
    flex51.end();
    step5.end();

    let step6 = group::Group::default_fill();
    let mut flex61 = group::Flex::default_fill().column();
    let mut flex62 = group::Flex::default().column();
    let mut image6 = Frame::default().with_size(600, 400);
    image6.set_image(Some(i5.clone()));
    let question6 = Frame::default()
        .with_size(80, 30)
        .with_label("Do you feel alone ?");
    flex62.fixed(&question6, 40);
    flex62.end();
    let flex63 = group::Flex::default().row();
    let mut never6_b = Button::default().with_label("never");
    let mut sometimes6_b = Button::default().with_label("sometimes");
    let mut often6_b = Button::default().with_label("often");
    let mut always6_b = Button::default().with_label("always");
    flex63.end();
    flex61.fixed(&flex63, 40);
    flex61.end();
    step6.end();

    let step7 = group::Group::default_fill();
    let mut flex71 = group::Flex::default_fill().column();
    let mut flex72 = group::Flex::default().column();
    let mut image7 = Frame::default().with_size(600, 400);
    image7.set_image(Some(i6.clone()));
    let question7 = Frame::default()
        .with_size(80, 30)
        .with_label("Do you feel loved ?");
    flex72.fixed(&question7, 40);
    flex72.end();
    let flex73 = group::Flex::default().row();
    let mut never7_b = Button::default().with_label("never");
    let mut sometimes7_b = Button::default().with_label("sometimes");
    let mut often7_b = Button::default().with_label("often");
    let mut always7_b = Button::default().with_label("always");
    flex73.end();
    flex71.fixed(&flex73, 40);
    flex71.end();
    step7.end();

    let step8 = group::Group::default_fill();
    let mut flex81 = group::Flex::default_fill().column();
    let mut flex82 = group::Flex::default().column();
    let mut image8 = Frame::default().with_size(600, 400);
    image8.set_image(Some(i7.clone()));
    let question8 = Frame::default()
        .with_size(80, 30)
        .with_label("Do you question the meaning of your life ?");
    flex82.fixed(&question8, 40);
    flex82.end();
    let flex83 = group::Flex::default().row();
    let mut never8_b = Button::default().with_label("never");
    let mut sometimes8_b = Button::default().with_label("sometimes");
    let mut often8_b = Button::default().with_label("often");
    let mut always8_b = Button::default().with_label("always");
    flex83.end();
    flex81.fixed(&flex83, 40);
    flex81.end();
    step8.end();

    let end = group::Group::default_fill();
    let end_flex = group::Flex::default_fill().column();
    let frres = Rc::new(RefCell::new(Frame::default()));
    let frcom = Rc::new(RefCell::new(Frame::default()));
    end_flex.end();
    end.end();

    wizard.end();

    start_btn.set_callback({
        let mut wiz = wizard.clone();
        move |_| wiz.next()
    });

    yes1_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 1;
            }
            wiz.next()
        }
    });

    no1_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb -= 50;
            }
            wiz.next()
        }
    });

    yes2_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 1;
            }
            wiz.next()
        }
    });

    no2_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb -= 50;
            }
            wiz.next()
        }
    });

    yes3_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 1;
            }
            wiz.next()
        }
    });

    no3_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb -= 50;
            }
            wiz.next()
        }
    });

    never4_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb -= 2;
            }
            wiz.next()
        }
    });

    sometimes4_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 1;
            }
            wiz.next()
        }
    });

    often4_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 4;
            }
            wiz.next()
        }
    });

    dontknow4_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb -= 10;
            }
            wiz.next()
        }
    });

    no5_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb -= 2;
            }
            wiz.next()
        }
    });

    yes5_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 4;
            }
            wiz.next()
        }
    });

    deftones5_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 15;
            }
            wiz.next()
        }
    });

    never6_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb -= 2;
            }
            wiz.next()
        }
    });

    sometimes6_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 2;
            }
            wiz.next()
        }
    });

    often6_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 6;
            }
            wiz.next()
        }
    });

    always6_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 10;
            }
            wiz.next()
        }
    });

    never7_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 15;
            }
            wiz.next()
        }
    });

    sometimes7_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 4;
            }
            wiz.next()
        }
    });

    often7_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 2;
            }
            wiz.next()
        }
    });

    always7_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        move |_| {
            {
                let mut hpb = hp.borrow_mut();
                *hpb -= 1;
            }
            wiz.next()
        }
    });

    never8_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        let fr = frres.clone();
        let frr = frres.clone();
        let frc = frcom.clone();
        move |_| {
            final_eval(hp.clone(), frr.clone(), frc.clone());
            {
                let mut hpb = hp.borrow_mut();
                let frb = fr.borrow_mut();
                *hpb -= 10;
            }
            wiz.next()
        }
    });

    sometimes8_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        let fr = frres.clone();
        let frr = frres.clone();
        let frc = frcom.clone();
        move |_| {
            final_eval(hp.clone(), frr.clone(), frc.clone());
            {
                let mut hpb = hp.borrow_mut();
                let frb = fr.borrow_mut();
                *hpb += 1;
            }
            wiz.next()
        }
    });

    often8_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        let fr = frres.clone();
        let frr = frres.clone();
        let frc = frcom.clone();
        move |_| {
            final_eval(hp.clone(), frr.clone(), frc.clone());
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 4;
            }
            wiz.next()
        }
    });

    always8_b.set_callback({
        let mut wiz = wizard.clone();
        let hp = human_point.clone();
        let frr = frres.clone();
        let frc = frcom.clone();
        move |_| {
            final_eval(hp.clone(), frr.clone(), frc.clone());
            {
                let mut hpb = hp.borrow_mut();
                *hpb += 10;
            }
            wiz.next()
        }
    });

    window.make_resizable(true);
    window.end();
    window.show();
    app.run().unwrap();
}
