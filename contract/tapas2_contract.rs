#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unreachable_code)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
mod module_cbc54463 {
    pub mod Tapas2_contract {
        use super::*;
        type DateTime = ();
        use fable_library_rust::List_::empty;
        use fable_library_rust::List_::ofArray;
        use fable_library_rust::List_::toArray;
        use fable_library_rust::Map_::find;
        use fable_library_rust::Map_::ofSeq;
        use fable_library_rust::Native_::Any;
        use fable_library_rust::Native_::Func0;
        use fable_library_rust::Native_::Func1;
        use fable_library_rust::Native_::LrcPtr;
        use fable_library_rust::Native_::MutCell;
        use fable_library_rust::Native_::OnceInit;
        use fable_library_rust::Native_::getNull;
        use fable_library_rust::Native_::on_startup;
        use fable_library_rust::Native_::unbox;
        use fable_library_rust::NativeArray_::new_array;
        use fable_library_rust::Option_::defaultValue;
        use fable_library_rust::Option_::map;
        use fable_library_rust::Seq_::ofList;
        use fable_library_rust::String_::append;
        use fable_library_rust::String_::getCharAt;
        use fable_library_rust::String_::printfn;
        use fable_library_rust::String_::sprintf;
        use fable_library_rust::String_::string;
        use fable_library_rust::String_::toLower;
        use fable_library_rust::String_::toString;
        use fable_library_rust::String_::trimEndChars;
        use fable_library_rust::String_::trimStartChars;
        use fable_library_rust::System::Collections::Generic::IEnumerable_1;
        pub trait IOsEnviron: core::fmt::Debug + core::fmt::Display {
            fn environ(&self) -> LrcPtr<dyn Any>;
        }
        impl<V: IOsEnviron + core::fmt::Debug + core::fmt::Display> IOsEnviron for LrcPtr<V> {
            #[inline]
            fn environ(&self) -> LrcPtr<dyn Any> {
                (**self).environ()
            }
        }
        pub mod TraceState {
            use super::*;
            pub fn trace_state() -> LrcPtr<
                MutCell<
                    Option<(
                        LrcPtr<Tapas2_contract::Mut0>,
                        LrcPtr<Tapas2_contract::Mut1>,
                        LrcPtr<Tapas2_contract::Mut2>,
                        LrcPtr<Tapas2_contract::Mut3>,
                        LrcPtr<Tapas2_contract::Mut4>,
                        Option<i64>,
                    )>,
                >,
            > {
                static trace_state: OnceInit<
                    LrcPtr<
                        MutCell<
                            Option<(
                                LrcPtr<Tapas2_contract::Mut0>,
                                LrcPtr<Tapas2_contract::Mut1>,
                                LrcPtr<Tapas2_contract::Mut2>,
                                LrcPtr<Tapas2_contract::Mut3>,
                                LrcPtr<Tapas2_contract::Mut4>,
                                Option<i64>,
                            )>,
                        >,
                    >,
                > = OnceInit::new();
                trace_state
                    .get_or_init(|| {
                        LrcPtr::new(MutCell::new(
                            None::<(
                                LrcPtr<Tapas2_contract::Mut0>,
                                LrcPtr<Tapas2_contract::Mut1>,
                                LrcPtr<Tapas2_contract::Mut2>,
                                LrcPtr<Tapas2_contract::Mut3>,
                                LrcPtr<Tapas2_contract::Mut4>,
                                Option<i64>,
                            )>,
                        ))
                    })
                    .clone()
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US0 {
            US0_0,
            US0_1,
            US0_2,
            US0_3,
            US0_4,
        }
        impl core::fmt::Display for US0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut0 {
            pub l0: MutCell<i64>,
        }
        impl core::fmt::Display for Mut0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub struct Mut1 {
            pub l0: MutCell<Func1<string, ()>>,
        }
        impl core::fmt::Display for Mut1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut2 {
            pub l0: MutCell<bool>,
        }
        impl core::fmt::Display for Mut2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut3 {
            pub l0: MutCell<string>,
        }
        impl core::fmt::Display for Mut3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut4 {
            pub l0: MutCell<Tapas2_contract::US0>,
        }
        impl core::fmt::Display for Mut4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US1 {
            US1_0(Tapas2_contract::US0),
            US1_1,
        }
        impl core::fmt::Display for US1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US2 {
            US2_0(i64),
            US2_1,
        }
        impl core::fmt::Display for US2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US3 {
            US3_0,
            US3_1,
            US3_2,
        }
        impl core::fmt::Display for US3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US4 {
            US4_0(Tapas2_contract::US3),
            US4_1(Tapas2_contract::US3),
            US4_2(Tapas2_contract::US3),
            US4_3(Tapas2_contract::US3),
            US4_4(Tapas2_contract::US3),
            US4_5(Tapas2_contract::US3),
            US4_6(Tapas2_contract::US3),
        }
        impl core::fmt::Display for US4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US5 {
            US5_0(string),
            US5_1,
        }
        impl core::fmt::Display for US5 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US6 {
            US6_0(
                LrcPtr<Tapas2_contract::Mut0>,
                LrcPtr<Tapas2_contract::Mut1>,
                LrcPtr<Tapas2_contract::Mut2>,
                LrcPtr<Tapas2_contract::Mut3>,
                LrcPtr<Tapas2_contract::Mut4>,
                Option<i64>,
            ),
            US6_1,
        }
        impl core::fmt::Display for US6 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        pub fn method3(v0_1: string) -> string {
            v0_1
        }
        pub fn method4() -> string {
            string("")
        }
        pub fn closure3(unitVar: (), v0_1: string) -> Tapas2_contract::US5 {
            Tapas2_contract::US5::US5_0(v0_1)
        }
        pub fn method5() -> Func1<string, Tapas2_contract::US5> {
            Func1::new(move |v: string| Tapas2_contract::closure3((), v))
        }
        pub fn method2(v0_1: string) -> string {
            panic!(
                "{}",
                sprintf!(
                    "env.get_environment_variable / target: {} / var: {}",
                    Tapas2_contract::US4::US4_4(Tapas2_contract::US3::US3_2),
                    v0_1
                ),
            )
        }
        pub fn method1() -> (Tapas2_contract::US1, Tapas2_contract::US2) {
            let v1: string = Tapas2_contract::method2(string("TRACE_LEVEL"));
            let v6: string = toLower(string("Critical"));
            let v13: string = toLower(string("Warning"));
            let v20: string = toLower(string("Info"));
            let v27: string = toLower(string("Debug"));
            let v34: string = toLower(string("Verbose"));
            let v41: Tapas2_contract::US1 = if string("Verbose") == (v1.clone()) {
                Tapas2_contract::US1::US1_0(Tapas2_contract::US0::US0_0)
            } else {
                Tapas2_contract::US1::US1_1
            };
            (
                match &v41 {
                    Tapas2_contract::US1::US1_0(v41_0_0) => Tapas2_contract::US1::US1_0(
                        match &v41 {
                            Tapas2_contract::US1::US1_0(x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                    ),
                    _ => {
                        let v48: Tapas2_contract::US1 = if string("Debug") == (v1.clone()) {
                            Tapas2_contract::US1::US1_0(Tapas2_contract::US0::US0_1)
                        } else {
                            Tapas2_contract::US1::US1_1
                        };
                        match &v48 {
                            Tapas2_contract::US1::US1_0(v48_0_0) => Tapas2_contract::US1::US1_0(
                                match &v48 {
                                    Tapas2_contract::US1::US1_0(x) => x.clone(),
                                    _ => unreachable!(),
                                }
                                .clone(),
                            ),
                            _ => {
                                let v55: Tapas2_contract::US1 = if string("Info") == (v1.clone()) {
                                    Tapas2_contract::US1::US1_0(Tapas2_contract::US0::US0_2)
                                } else {
                                    Tapas2_contract::US1::US1_1
                                };
                                match &v55 {
                                    Tapas2_contract::US1::US1_0(v55_0_0) => {
                                        Tapas2_contract::US1::US1_0(
                                            match &v55 {
                                                Tapas2_contract::US1::US1_0(x) => x.clone(),
                                                _ => unreachable!(),
                                            }
                                            .clone(),
                                        )
                                    }
                                    _ => {
                                        let v62: Tapas2_contract::US1 = if string("Warning")
                                            == (v1.clone())
                                        {
                                            Tapas2_contract::US1::US1_0(Tapas2_contract::US0::US0_3)
                                        } else {
                                            Tapas2_contract::US1::US1_1
                                        };
                                        match &v62 {
                                            Tapas2_contract::US1::US1_0(v62_0_0) => {
                                                Tapas2_contract::US1::US1_0(
                                                    match &v62 {
                                                        Tapas2_contract::US1::US1_0(x) => x.clone(),
                                                        _ => unreachable!(),
                                                    }
                                                    .clone(),
                                                )
                                            }
                                            _ => {
                                                let v69: Tapas2_contract::US1 =
                                                    if string("Critical") == (v1.clone()) {
                                                        Tapas2_contract::US1::US1_0(
                                                            Tapas2_contract::US0::US0_4,
                                                        )
                                                    } else {
                                                        Tapas2_contract::US1::US1_1
                                                    };
                                                match &v69 {
                                                    Tapas2_contract::US1::US1_0(v69_0_0) => {
                                                        Tapas2_contract::US1::US1_0(
                                                            match &v69 {
                                                                Tapas2_contract::US1::US1_0(x) => {
                                                                    x.clone()
                                                                }
                                                                _ => unreachable!(),
                                                            }
                                                            .clone(),
                                                        )
                                                    }
                                                    _ => {
                                                        let v76: Tapas2_contract::US1 =
                                                            if (v34.clone()) == (v1.clone()) {
                                                                Tapas2_contract::US1::US1_0(
                                                                    Tapas2_contract::US0::US0_0,
                                                                )
                                                            } else {
                                                                Tapas2_contract::US1::US1_1
                                                            };
                                                        match &v76 {
                                                            Tapas2_contract::US1::US1_0(
                                                                v76_0_0,
                                                            ) => Tapas2_contract::US1::US1_0(
                                                                match &v76 {
                                                                    Tapas2_contract::US1::US1_0(
                                                                        x,
                                                                    ) => x.clone(),
                                                                    _ => unreachable!(),
                                                                }
                                                                .clone(),
                                                            ),
                                                            _ => {
                                                                let v83: Tapas2_contract::US1 =
                                                                    if (v27.clone()) == (v1.clone())
                                                                    {
                                                                        Tapas2_contract::US1::US1_0(Tapas2_contract::US0::US0_1)
                                                                    } else {
                                                                        Tapas2_contract::US1::US1_1
                                                                    };
                                                                match &v83 {
                                                                 Tapas2_contract::US1::US1_0(v83_0_0)
                                                                 =>
                                                                 Tapas2_contract::US1::US1_0(match &v83
                                                                                                 {
                                                                                                 Tapas2_contract::US1::US1_0(x)
                                                                                                 =>
                                                                                                 x.clone(),
                                                                                                 _
                                                                                                 =>
                                                                                                 unreachable!(),
                                                                                             }.clone()),
                                                                 _ => {
                                                                     let v90:
                                                                             Tapas2_contract::US1 =
                                                                         if (v20.clone())
                                                                                ==
                                                                                (v1.clone())
                                                                            {
                                                                             Tapas2_contract::US1::US1_0(Tapas2_contract::US0::US0_2)
                                                                         } else {
                                                                             Tapas2_contract::US1::US1_1
                                                                         };
                                                                     match &v90
                                                                         {
                                                                         Tapas2_contract::US1::US1_0(v90_0_0)
                                                                         =>
                                                                         Tapas2_contract::US1::US1_0(match &v90
                                                                                                         {
                                                                                                         Tapas2_contract::US1::US1_0(x)
                                                                                                         =>
                                                                                                         x.clone(),
                                                                                                         _
                                                                                                         =>
                                                                                                         unreachable!(),
                                                                                                     }.clone()),
                                                                         _ =>
                                                                         {
                                                                             let v97:
                                                                                     Tapas2_contract::US1 =
                                                                                 if (v13.clone())
                                                                                        ==
                                                                                        (v1.clone())
                                                                                    {
                                                                                     Tapas2_contract::US1::US1_0(Tapas2_contract::US0::US0_3)
                                                                                 } else {
                                                                                     Tapas2_contract::US1::US1_1
                                                                                 };
                                                                             match &v97
                                                                                 {
                                                                                 Tapas2_contract::US1::US1_0(v97_0_0)
                                                                                 =>
                                                                                 Tapas2_contract::US1::US1_0(match &v97
                                                                                                                 {
                                                                                                                 Tapas2_contract::US1::US1_0(x)
                                                                                                                 =>
                                                                                                                 x.clone(),
                                                                                                                 _
                                                                                                                 =>
                                                                                                                 unreachable!(),
                                                                                                             }.clone()),
                                                                                 _
                                                                                 =>
                                                                                 {
                                                                                     let v104:
                                                                                             Tapas2_contract::US1 =
                                                                                         if (v6.clone())
                                                                                                ==
                                                                                                (v1.clone())
                                                                                            {
                                                                                             Tapas2_contract::US1::US1_0(Tapas2_contract::US0::US0_4)
                                                                                         } else {
                                                                                             Tapas2_contract::US1::US1_1
                                                                                         };
                                                                                     match &v104
                                                                                         {
                                                                                         Tapas2_contract::US1::US1_0(v104_0_0)
                                                                                         =>
                                                                                         Tapas2_contract::US1::US1_0(match &v104
                                                                                                                         {
                                                                                                                         Tapas2_contract::US1::US1_0(x)
                                                                                                                         =>
                                                                                                                         x.clone(),
                                                                                                                         _
                                                                                                                         =>
                                                                                                                         unreachable!(),
                                                                                                                     }.clone()),
                                                                                         _
                                                                                         =>
                                                                                         Tapas2_contract::US1::US1_1,
                                                                                     }
                                                                                 }
                                                                             }
                                                                         }
                                                                     }
                                                                 }
                                                             }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                if (Tapas2_contract::method2(string("AUTOMATION"))) != string("True") {
                    Tapas2_contract::US2::US2_1
                } else {
                    let v147: DateTime = unbox::<DateTime>(fable_library_rust::Native_::getZero());
                    Tapas2_contract::US2::US2_0(
                        unbox::<i64>(fable_library_rust::Native_::getZero()),
                    )
                },
            )
        }
        pub fn closure4(unitVar: (), v0_1: string) {
            ();
        }
        pub fn method0(
            v0_1: Tapas2_contract::US0,
        ) -> (
            LrcPtr<Tapas2_contract::Mut0>,
            LrcPtr<Tapas2_contract::Mut1>,
            LrcPtr<Tapas2_contract::Mut2>,
            LrcPtr<Tapas2_contract::Mut3>,
            LrcPtr<Tapas2_contract::Mut4>,
            Option<i64>,
        ) {
            let v94: string = string("option_env!(\"AUTOMATION\").unwrap_or(\"\")");
            let v95: &str = option_env!("AUTOMATION").unwrap_or("");
            let v106: std::string::String = String::from(v95);
            let _run_target_args__v3: (Tapas2_contract::US1, Tapas2_contract::US2) = (
                Tapas2_contract::US1::US1_1,
                if (fable_library_rust::String_::fromString(v106)) != string("True") {
                    Tapas2_contract::US2::US2_1
                } else {
                    Tapas2_contract::US2::US2_0(near_sdk::env::block_timestamp() as i64)
                },
            );
            let v185: Tapas2_contract::US2 = _run_target_args__v3.1.clone();
            let v184: Tapas2_contract::US1 = _run_target_args__v3.0.clone();
            (
                LrcPtr::new(Tapas2_contract::Mut0 {
                    l0: MutCell::new(1_i64),
                }),
                LrcPtr::new(Tapas2_contract::Mut1 {
                    l0: MutCell::new(Func1::new(move |v: string| {
                        Tapas2_contract::closure4((), v)
                    })),
                }),
                LrcPtr::new(Tapas2_contract::Mut2 {
                    l0: MutCell::new(true),
                }),
                LrcPtr::new(Tapas2_contract::Mut3 {
                    l0: MutCell::new(string("")),
                }),
                LrcPtr::new(Tapas2_contract::Mut4 {
                    l0: MutCell::new(match &v184 {
                        Tapas2_contract::US1::US1_0(v184_0_0) => match &v184 {
                            Tapas2_contract::US1::US1_0(x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                        _ => v0_1.clone(),
                    }),
                }),
                match &v185 {
                    Tapas2_contract::US2::US2_0(v185_0_0) => Some(match &v185 {
                        Tapas2_contract::US2::US2_0(x) => x.clone(),
                        _ => unreachable!(),
                    }),
                    _ => None::<i64>,
                },
            )
        }
        pub fn closure2(unitVar: (), unitVar_1: ()) {
            if Tapas2_contract::TraceState::trace_state()
                .get()
                .clone()
                .is_none()
            {
                let patternInput: (
                    LrcPtr<Tapas2_contract::Mut0>,
                    LrcPtr<Tapas2_contract::Mut1>,
                    LrcPtr<Tapas2_contract::Mut2>,
                    LrcPtr<Tapas2_contract::Mut3>,
                    LrcPtr<Tapas2_contract::Mut4>,
                    Option<i64>,
                ) = Tapas2_contract::method0(Tapas2_contract::US0::US0_0);
                Tapas2_contract::TraceState::trace_state().set(Some((
                    patternInput.0.clone(),
                    patternInput.1.clone(),
                    patternInput.2.clone(),
                    patternInput.3.clone(),
                    patternInput.4.clone(),
                    patternInput.5.clone(),
                )));
                ()
            };
        }
        pub fn closure5(unitVar: (), v0_1: i64) -> Tapas2_contract::US2 {
            Tapas2_contract::US2::US2_0(v0_1)
        }
        pub fn method7() -> Func1<i64, Tapas2_contract::US2> {
            Func1::new(move |v: i64| Tapas2_contract::closure5((), v))
        }
        pub fn method8() -> string {
            string("hh:mm:ss")
        }
        pub fn method9() -> string {
            string("HH:mm:ss")
        }
        pub fn method6(
            v0_1: LrcPtr<Tapas2_contract::Mut0>,
            v1: LrcPtr<Tapas2_contract::Mut1>,
            v2: LrcPtr<Tapas2_contract::Mut2>,
            v3: LrcPtr<Tapas2_contract::Mut3>,
            v4: LrcPtr<Tapas2_contract::Mut4>,
            v5: Option<i64>,
        ) -> string {
            let v569: u64 = near_sdk::env::block_timestamp();
            let v594: Tapas2_contract::US2 = defaultValue(
                Tapas2_contract::US2::US2_1,
                map(Tapas2_contract::method7(), v5),
            );
            let v607: u64 = (match &v594 {
                Tapas2_contract::US2::US2_0(v594_0_0) => {
                    (v569)
                        - (match &v594 {
                            Tapas2_contract::US2::US2_0(x) => x.clone(),
                            _ => unreachable!(),
                        } as u64)
                }
                _ => v569,
            }) / 1000000000_u64;
            let v608: u64 = (v607) % 60_u64;
            let v610: u64 = ((v607) / 60_u64) % 60_u64;
            let v612: u64 = ((v607) / 3600_u64) % 24_u64;
            let v614: std::string::String = format!("{:02}:{:02}:{:02}", v612, v610, v608);
            fable_library_rust::String_::fromString(v614)
        }
        pub fn method12() -> string {
            string("")
        }
        pub fn closure6(v0_1: LrcPtr<Tapas2_contract::Mut3>, v1: string, unitVar: ()) {
            let v4: string = append((v0_1.l0.get().clone()), (v1));
            v0_1.l0.set(v4);
            ()
        }
        pub fn method11(v0_1: char) -> string {
            let v2: LrcPtr<Tapas2_contract::Mut3> = LrcPtr::new(Tapas2_contract::Mut3 {
                l0: MutCell::new(Tapas2_contract::method12()),
            });
            let v17: () = {
                Tapas2_contract::closure6(v2.clone(), sprintf!("{}", v0_1), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method13() -> string {
            string("\u{001b}[0m")
        }
        pub fn method10() -> string {
            let v8: string = Tapas2_contract::method11(getCharAt(toLower(string("Debug")), 0_i32));
            let v115: &str = inline_colorization::color_bright_blue;
            let v126: &str = &*v8;
            let v153: &str = inline_colorization::color_reset;
            let v155: std::string::String = format!("{}{}{}", v115, v126, v153);
            fable_library_rust::String_::fromString(v155)
        }
        pub fn method15(v0_1: i32, v1: string) -> string {
            let v3: LrcPtr<Tapas2_contract::Mut3> = LrcPtr::new(Tapas2_contract::Mut3 {
                l0: MutCell::new(Tapas2_contract::method12()),
            });
            let v17: () = {
                Tapas2_contract::closure6(v3.clone(), string("{ "), ());
                ()
            };
            let v36: () = {
                Tapas2_contract::closure6(v3.clone(), string("seed_excess_len"), ());
                ()
            };
            let v55: () = {
                Tapas2_contract::closure6(v3.clone(), string(" = "), ());
                ()
            };
            let v75: () = {
                Tapas2_contract::closure6(v3.clone(), sprintf!("{}", v0_1), ());
                ()
            };
            let v94: () = {
                Tapas2_contract::closure6(v3.clone(), string("; "), ());
                ()
            };
            let v113: () = {
                Tapas2_contract::closure6(v3.clone(), string("seed_excess"), ());
                ()
            };
            let v130: () = {
                Tapas2_contract::closure6(v3.clone(), string(" = "), ());
                ()
            };
            let v147: () = {
                Tapas2_contract::closure6(v3.clone(), v1, ());
                ()
            };
            let v166: () = {
                Tapas2_contract::closure6(v3.clone(), string(" }"), ());
                ()
            };
            v3.l0.get().clone()
        }
        pub fn method16(v0_1: string) -> string {
            trimEndChars(
                trimStartChars(v0_1, toArray(empty::<char>())),
                toArray(ofArray(new_array(&[' ', '/']))),
            )
        }
        pub fn method14(
            v0_1: LrcPtr<Tapas2_contract::Mut0>,
            v1: LrcPtr<Tapas2_contract::Mut1>,
            v2: LrcPtr<Tapas2_contract::Mut2>,
            v3: LrcPtr<Tapas2_contract::Mut3>,
            v4: LrcPtr<Tapas2_contract::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: i32,
            v9: string,
        ) -> string {
            let v10: string = Tapas2_contract::method15(v8, v9);
            Tapas2_contract::method16(append(
                (append(
                    (append(
                        (append(
                            (append(
                                (append((append((append((v6), string(" "))), (v7))), string(" #"))),
                                (toString(v0_1.l0.get().clone())),
                            )),
                            string(" "),
                        )),
                        string("tapas2_contract.contribute_seed"),
                    )),
                    string(" / "),
                )),
                (v10),
            ))
        }
        pub fn closure7(v0_1: LrcPtr<Tapas2_contract::Mut0>, unitVar: ()) {
            let v2: i64 = (v0_1.l0.get().clone()) + 1_i64;
            v0_1.l0.set(v2);
            ()
        }
        pub fn closure9(v0_1: string, unitVar: ()) {
            printfn!("{0}", v0_1);
        }
        pub fn closure8(unitVar: (), v0_1: string) {
            let v4: () = {
                Tapas2_contract::closure9(v0_1, ());
                ()
            };
            ()
        }
        pub fn closure1(v0_1: i32, v1: Vec<u8>, unitVar: ()) {
            fn v17() {
                Tapas2_contract::closure2((), ());
            }
            let v18: () = {
                v17();
                ()
            };
            let patternInput: (
                LrcPtr<Tapas2_contract::Mut0>,
                LrcPtr<Tapas2_contract::Mut1>,
                LrcPtr<Tapas2_contract::Mut2>,
                LrcPtr<Tapas2_contract::Mut3>,
                LrcPtr<Tapas2_contract::Mut4>,
                Option<i64>,
            ) = Tapas2_contract::TraceState::trace_state()
                .get()
                .clone()
                .unwrap();
            let v59: Tapas2_contract::US0 = (patternInput.4.clone()).l0.get().clone();
            let v303: Tapas2_contract::US6 =
                if (if ((patternInput.2.clone()).l0.get().clone()) == false {
                    false
                } else {
                    1_i32
                        >= (find(
                            v59,
                            ofSeq(ofList(ofArray(new_array(&[
                                LrcPtr::new((Tapas2_contract::US0::US0_0, 0_i32)),
                                LrcPtr::new((Tapas2_contract::US0::US0_1, 1_i32)),
                                LrcPtr::new((Tapas2_contract::US0::US0_2, 2_i32)),
                                LrcPtr::new((Tapas2_contract::US0::US0_3, 3_i32)),
                                LrcPtr::new((Tapas2_contract::US0::US0_4, 4_i32)),
                            ])))),
                        ))
                }) == false
                {
                    Tapas2_contract::US6::US6_1
                } else {
                    let v82: () = {
                        v17();
                        ()
                    };
                    let patternInput_1: (
                        LrcPtr<Tapas2_contract::Mut0>,
                        LrcPtr<Tapas2_contract::Mut1>,
                        LrcPtr<Tapas2_contract::Mut2>,
                        LrcPtr<Tapas2_contract::Mut3>,
                        LrcPtr<Tapas2_contract::Mut4>,
                        Option<i64>,
                    ) = Tapas2_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v110: Option<i64> = patternInput_1.5.clone();
                    let v109: LrcPtr<Tapas2_contract::Mut4> = patternInput_1.4.clone();
                    let v108: LrcPtr<Tapas2_contract::Mut3> = patternInput_1.3.clone();
                    let v107: LrcPtr<Tapas2_contract::Mut2> = patternInput_1.2.clone();
                    let v106: LrcPtr<Tapas2_contract::Mut1> = patternInput_1.1.clone();
                    let v105: LrcPtr<Tapas2_contract::Mut0> = patternInput_1.0.clone();
                    let v129: string = Tapas2_contract::method14(
                        v105.clone(),
                        v106.clone(),
                        v107.clone(),
                        v108.clone(),
                        v109.clone(),
                        v110.clone(),
                        Tapas2_contract::method6(v105, v106, v107, v108, v109, v110),
                        Tapas2_contract::method10(),
                        v0_1,
                        sprintf!("{:?}", v1),
                    );
                    let v145: () = {
                        v17();
                        ()
                    };
                    let patternInput_2: (
                        LrcPtr<Tapas2_contract::Mut0>,
                        LrcPtr<Tapas2_contract::Mut1>,
                        LrcPtr<Tapas2_contract::Mut2>,
                        LrcPtr<Tapas2_contract::Mut3>,
                        LrcPtr<Tapas2_contract::Mut4>,
                        Option<i64>,
                    ) = Tapas2_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v171: LrcPtr<Tapas2_contract::Mut3> = patternInput_2.3.clone();
                    let v169: LrcPtr<Tapas2_contract::Mut1> = patternInput_2.1.clone();
                    let v168: LrcPtr<Tapas2_contract::Mut0> = patternInput_2.0.clone();
                    let v190: () = {
                        Tapas2_contract::closure7(v168.clone(), ());
                        ()
                    };
                    let v213: string = if (v171.l0.get().clone()) == string("") {
                        v129.clone()
                    } else {
                        if (v129.clone()) == string("") {
                            v171.l0.get().clone()
                        } else {
                            append(
                                (append((v171.l0.get().clone()), string("\n"))),
                                (v129.clone()),
                            )
                        }
                    };
                    let v224: &str = &*v213.clone();
                    let v251 = v224.chars();
                    let v253 = v251;
                    let v255: Vec<char> = v253.collect::<Vec<_>>();
                    let v257: Vec<Vec<char>> = v255
                        .chunks(15000)
                        .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    let v259: bool = true;
                    let _vec_map: Vec<_> = v257
                        .into_iter()
                        .map(|x| {
                            //;
                            let v261: Vec<char> = x;
                            let v263: std::string::String = String::from_iter(v261);
                            let v265: bool = true;
                            v263
                        })
                        .collect::<Vec<_>>();
                    let v267: Vec<std::string::String> = _vec_map;
                    if if (v129.clone()) != string("") {
                        (v267.clone().len() as i32) <= 1_i32
                    } else {
                        false
                    } {
                        v171.l0.set(v213);
                        ()
                    } else {
                        v171.l0.set(string(""));
                        {
                            let v292: bool = true;
                            v267.into_iter().for_each(|x| {
                                //;
                                let v294: std::string::String = x;
                                let v296: bool = true;
                                near_sdk::log!("{}", v294);
                                let v298: bool = true;
                                let v300: bool = true;
                            }); //;
                            ()
                        }
                    }
                    (v169.l0.get().clone())(v129);
                    Tapas2_contract::US6::US6_0(
                        v168,
                        v169,
                        patternInput_2.2.clone(),
                        v171,
                        patternInput_2.4.clone(),
                        patternInput_2.5.clone(),
                    )
                };
            ()
        }
        pub fn closure10(unitVar: (), unitVar_1: ()) {
            ();
        }
        pub fn method17() -> Func0<()> {
            Func0::new(move || Tapas2_contract::closure10((), ()))
        }
        pub fn closure11(v0_1: Func0<()>, unitVar: ()) {
            fn v16() {
                Tapas2_contract::closure2((), ());
            }
            let v17: () = {
                v16();
                ()
            };
            let patternInput: (
                LrcPtr<Tapas2_contract::Mut0>,
                LrcPtr<Tapas2_contract::Mut1>,
                LrcPtr<Tapas2_contract::Mut2>,
                LrcPtr<Tapas2_contract::Mut3>,
                LrcPtr<Tapas2_contract::Mut4>,
                Option<i64>,
            ) = Tapas2_contract::TraceState::trace_state()
                .get()
                .clone()
                .unwrap();
            let v58: Tapas2_contract::US0 = (patternInput.4.clone()).l0.get().clone();
            let v296: Tapas2_contract::US6 =
                if (if ((patternInput.2.clone()).l0.get().clone()) == false {
                    false
                } else {
                    1_i32
                        >= (find(
                            v58,
                            ofSeq(ofList(ofArray(new_array(&[
                                LrcPtr::new((Tapas2_contract::US0::US0_0, 0_i32)),
                                LrcPtr::new((Tapas2_contract::US0::US0_1, 1_i32)),
                                LrcPtr::new((Tapas2_contract::US0::US0_2, 2_i32)),
                                LrcPtr::new((Tapas2_contract::US0::US0_3, 3_i32)),
                                LrcPtr::new((Tapas2_contract::US0::US0_4, 4_i32)),
                            ])))),
                        ))
                }) == false
                {
                    Tapas2_contract::US6::US6_1
                } else {
                    let v81: () = {
                        v16();
                        ()
                    };
                    let patternInput_1: (
                        LrcPtr<Tapas2_contract::Mut0>,
                        LrcPtr<Tapas2_contract::Mut1>,
                        LrcPtr<Tapas2_contract::Mut2>,
                        LrcPtr<Tapas2_contract::Mut3>,
                        LrcPtr<Tapas2_contract::Mut4>,
                        Option<i64>,
                    ) = Tapas2_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v122: string = Tapas2_contract::method6(
                        patternInput_1.0.clone(),
                        patternInput_1.1.clone(),
                        patternInput_1.2.clone(),
                        patternInput_1.3.clone(),
                        patternInput_1.4.clone(),
                        patternInput_1.5.clone(),
                    );
                    let v123: string = Tapas2_contract::method10();
                    let v139: () = {
                        v16();
                        ()
                    };
                    let patternInput_2: (
                        LrcPtr<Tapas2_contract::Mut0>,
                        LrcPtr<Tapas2_contract::Mut1>,
                        LrcPtr<Tapas2_contract::Mut2>,
                        LrcPtr<Tapas2_contract::Mut3>,
                        LrcPtr<Tapas2_contract::Mut4>,
                        Option<i64>,
                    ) = Tapas2_contract::TraceState::trace_state()
                        .get()
                        .clone()
                        .unwrap();
                    let v165: LrcPtr<Tapas2_contract::Mut3> = patternInput_2.3.clone();
                    let v163: LrcPtr<Tapas2_contract::Mut1> = patternInput_2.1.clone();
                    let v162: LrcPtr<Tapas2_contract::Mut0> = patternInput_2.0.clone();
                    let v184: () = {
                        Tapas2_contract::closure7(v162.clone(), ());
                        ()
                    };
                    let v208: string = if (v165.l0.get().clone()) == string("") {
                        string("")
                    } else {
                        v165.l0.get().clone()
                    };
                    let v219: &str = &*v208;
                    let v246 = v219.chars();
                    let v248 = v246;
                    let v250: Vec<char> = v248.collect::<Vec<_>>();
                    let v252: Vec<Vec<char>> = v250
                        .chunks(15000)
                        .map(|x| x.into_iter().map(|x| x.clone()).collect::<Vec<_>>())
                        .collect::<Vec<_>>();
                    let v254: bool = true;
                    let _vec_map: Vec<_> = v252
                        .into_iter()
                        .map(|x| {
                            //;
                            let v256: Vec<char> = x;
                            let v258: std::string::String = String::from_iter(v256);
                            let v260: bool = true;
                            v258
                        })
                        .collect::<Vec<_>>();
                    let v262: Vec<std::string::String> = _vec_map;
                    v165.l0.set(string(""));
                    {
                        let v285: bool = true;
                        v262.into_iter().for_each(|x| {
                            //;
                            let v287: std::string::String = x;
                            let v289: bool = true;
                            near_sdk::log!("{}", v287);
                            let v291: bool = true;
                            let v293: bool = true;
                        }); //;
                        ()
                    }
                    (v163.l0.get().clone())(string(""));
                    Tapas2_contract::US6::US6_0(
                        v162,
                        v163,
                        patternInput_2.2.clone(),
                        v165,
                        patternInput_2.4.clone(),
                        patternInput_2.5.clone(),
                    )
                };
            ()
        }
        pub fn closure0(unitVar: (), unitVar_1: ()) {} //;
        #[derive(
            //;
            near_sdk::PanicOnDefault, //;
            borsh::BorshDeserialize,  //;
            borsh::BorshSerialize,    //;
        )] //;
        pub struct OldState {
            //;
            version: u32,                            //;
            seeds: near_sdk::store::vec::Vector<u8>, //;
        } //;
        #[near_sdk::near_bindgen] //;
        #[derive(
            //;
            near_sdk::PanicOnDefault, //;
            borsh::BorshDeserialize,  //;
            borsh::BorshSerialize,    //;
        )] //;
        pub struct State(
            //;
            /*;
            {
                let v19: */
            (u32, near_sdk::store::vec::Vector<u8>), /* =
                                                         fable_library_rust::Native_::getZero::<()>();
                                                     */
        );
        impl From<OldState> for State {
            //;
            fn from(old_state: OldState) -> Self {
                //;
                Self((old_state.version + 1, old_state.seeds)) //;
            } //;
        } //;
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            #[init] //;
            pub fn new() -> Self {
                // 1;
                {
                    let v33: bool = true; /*;
                    let v35: */
                    () /* = fable_library_rust::Native_::getZero();
                    let v39: bool = true; */;
                    let v41: string = string("b\"seeds\"");
                    let v42: &[u8] = b"seeds";
                    {
                        let x: (u32, near_sdk::store::vec::Vector<u8>) =
                            (2_u32, near_sdk::store::vec::Vector::new(v42));
                        Self(x) // x
                    }
                } // 2.;
            } // 1.;
        } /* c;
        {
        let v49: bool =
        true; // ??? / i: 1uy / i': 1uy / acc: 0uy / n: 2uy;
        let v51: bool =
        true; */
 // ???? / i: 1uy / i': 2uy / acc: 0uy / n: 2uy;
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            pub fn contribute_seed(&mut self, seed: Vec<u8>) {
                //;
                {
                    let v56: &mut near_sdk::store::vec::Vector<u8> = &mut self.0.1;
                    let v58: Vec<u8> = seed;
                    let v60: bool = true;
                    v56.extend(v58); //;
                    let v93: i32 = (v56.len() as i32) - (100_i32);
                    if (v93) > 0_i32 {
                        let v414: () = {
                            Tapas2_contract::closure1(
                                v93,
                                v56.drain(0..v93 as u32).collect::<Vec<_>>(),
                                (),
                            );
                            ()
                        };
                        ()
                    }
                    {
                        let v1045: () = {
                            Tapas2_contract::closure11(Tapas2_contract::method17(), ());
                            ()
                        };
                    } //;
                } //;
            } //;
        } /* c;
        {
        let v1363: bool =
        true; // ??? / i: 2uy / i': 1uy / acc: 2uy / n: 3uy;
        let v1365: bool =
        true; */
 // ???? / i: 2uy / i': 2uy / acc: 2uy / n: 3uy;
        #[near_sdk::near_bindgen] //;
        impl State {
            //;
            pub fn contribute_seed_borsh(&mut self, #[serializer(borsh)] seed: Vec<u8>) {
                //;
                self.contribute_seed(seed) //;
            } //;
        } /* c;
        {
        let v1373: bool =
        true; // ??? / i: 3uy / i': 1uy / acc: 5uy / n: 1uy;
        let v1375: bool =
        true; */
 // ???? / i: 3uy / i': 2uy / acc: 5uy / n: 1uy;
        fn _main() //;
        //;
        {
            let v1379: bool = true;
            {
                (); // ?? / i': 1uy / n: 6uy;
                let v1381: bool = true;
                {
                    (); // ?? / i': 2uy / n: 6uy;
                    let v1383: bool = true;
                    {
                        (); // ?? / i': 3uy / n: 6uy;
                        let v1385: bool = true;
                        {
                            (); // ?? / i': 4uy / n: 6uy;
                            let v1387: bool = true;
                            {
                                (); // ?? / i': 5uy / n: 6uy;
                                let v1389: bool = true;
                                {
                                    (); // ?? / i': 6uy / n: 6uy;
                                    let v1391: bool = true;
                                    {
                                        {
                                            (); // ? / i': 7uy / n: 6uy;
                                            ()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        pub fn v0() -> Func0<()> {
            static v0: OnceInit<Func0<()>> = OnceInit::new();
            v0.get_or_init(|| Func0::new(move || Tapas2_contract::closure0((), ())))
                .clone()
        }
        on_startup!(());
    }
}
pub use module_cbc54463::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/async__contract.rs"]
mod module_2335f2f5;
pub use module_2335f2f5::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/common_contract.rs"]
mod module_652e6d81;
pub use module_652e6d81::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/crypto_contract.rs"]
mod module_dd5f95ef;
pub use module_dd5f95ef::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/date_time_contract.rs"]
mod module_ca5e6cb2;
pub use module_ca5e6cb2::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/file_system_contract.rs"]
mod module_5ab1faf0;
pub use module_5ab1faf0::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/lib_contract.rs"]
mod module_b386774b;
pub use module_b386774b::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/networking_contract.rs"]
mod module_ce497f72;
pub use module_ce497f72::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/platform_contract.rs"]
mod module_9a61edd3;
pub use module_9a61edd3::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/runtime_contract.rs"]
mod module_502d7e30;
pub use module_502d7e30::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/sm_contract.rs"]
mod module_34f67952;
pub use module_34f67952::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/threading_contract.rs"]
mod module_11c0c5c2;
pub use module_11c0c5c2::*;
#[path = "../deps/polyglot/deps/spiral/lib/spiral/trace_contract.rs"]
mod module_28ecba0d;
pub use module_28ecba0d::*;
#[path = "../deps/polyglot/lib/fsharp/Common_contract.rs"]
mod module_ad43931;
pub use module_ad43931::*;
pub mod Polyglot {
    pub use crate::module_ad43931::Polyglot::*;
}
