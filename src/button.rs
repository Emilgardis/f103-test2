use stm32f103xx as f103;


// External button is on PB7
pub fn init(gpiob: &f103::GPIOB, rcc: &f103::RCC, exti: &f103::EXTI, afio: &f103::AFIO) {
    rcc.apb2enr.modify(|_,w| { 
        w.iopben().enabled()
        .afioen().enabled() 
    });

    gpiob.crl.modify(|_,w| w
                        .mode7().input()
                        .cnf7().bits(0b10) // Input with pull-up / pull-down, default pull-up, set 1 in PxODR for  p-down
    );
    

    // 9.4.3
    // http://www.st.com/content/ccc/resource/technical/document/reference_manual/59/b9/ba/7f/11/af/43/d5/CD00171190.pdf/files/CD00171190.pdf/jcr:content/translations/en.CD00171190.pdf
    // PB[x] is 0001
    afio.exticr2.modify(|_,w| unsafe { w.exti7().bits(0b0001) });
    // http://www.hertaville.com/external-interrupts-on-the-stm32f0.html

    exti.imr.modify(|_,w| w.mr7().set());
    exti.emr.modify(|_,w| w.mr7().set());
    exti.rtsr.modify(|_,w| w.tr7().set());
    //exti.ftsr.modify(|_,w| w.tr7().set());
}
