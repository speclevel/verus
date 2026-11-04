#![feature(rustc_private)]
#[macro_use]
mod common;
use common::*;

test_verify_one_file! {
    #[test] scoped_trust_and_private_access verus_code! {
        mod private {
            #[verifier::external_body]
            fn trusted() -> (v: u8) ensures v == 7 { 7 }

            pub struct Value { field: u8 }

            impl Value {
                #[verifier::no_cheating]
                fn checked(&self) -> (v: u8) ensures v == self.field {
                    let t = trusted();
                    assert(t == 7);
                    self.field
                }
            }

            #[verifier::no_cheating]
            fn check() {
                let x = Value { field: 5 };
                let r = x.checked();
                assert(r == 5);
            }
        }
    } => Ok(())
}

test_verify_one_file! {
    #[test] scoped_assume verus_code! {
        #[verifier::no_cheating]
        proof fn checked() { assume(false); }
    } => Err(e) => assert_vir_error_msg(e, "assume/admit not allowed with #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_admit verus_code! {
        #[verifier::no_cheating]
        proof fn checked() { admit(); }
    } => Err(e) => assert_vir_error_msg(e, "assume/admit not allowed with #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_aliased_assume verus_code! {
        use verus_builtin::assume_ as hidden;
        #[verifier::no_cheating]
        proof fn checked() { hidden(false); }
    } => Err(e) => assert_vir_error_msg(e, "assume/admit not allowed with #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_macro_assume verus_code! {
        macro_rules! hidden { () => { verus_builtin::assume_(false) } }
        #[verifier::no_cheating]
        proof fn checked() { hidden!(); }
    } => Err(e) => assert_vir_error_msg(e, "assume/admit not allowed with #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_nested_function verus_code! {
        #[verifier::no_cheating]
        proof fn checked() {
            proof fn nested() { assume(false); }
            nested();
        }
    } => Err(e) => assert_vir_error_msg(e, "assume/admit not allowed with #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_closure verus_code! {
        #[verifier::no_cheating]
        fn checked() {
            let f = || { proof { assume(false); } };
        }
    } => Err(e) => assert_vir_error_msg(e, "assume/admit not allowed with #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_external_body verus_code! {
        #[verifier::no_cheating]
        #[verifier::external_body]
        proof fn checked() {}
    } => Err(e) => assert_vir_error_msg(e, "trusted declarations are not allowed within #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_external verus_code! {
        #[verifier::no_cheating]
        #[verifier::external]
        fn checked() {}
    } => Err(e) => assert_vir_error_msg(e, "external code and trusted declarations are not allowed within #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_inside_external_module verus_code! {
        #[verifier::external]
        mod ignored {
            #[verifier::no_cheating]
            fn checked() {}
        }
    } => Err(e) => assert_vir_error_msg(e, "external code and trusted declarations are not allowed within #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_macro_external_helper verus_code! {
        macro_rules! hidden {
            () => { #[verifier::external] fn nested() {} }
        }
        #[verifier::no_cheating]
        fn checked() { hidden!(); }
    } => Err(e) => assert_vir_error_msg(e, "external code and trusted declarations are not allowed within #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_nested_external_body verus_code! {
        #[verifier::no_cheating]
        proof fn checked() {
            #[verifier::external_body]
            proof fn nested() ensures false {}
            nested();
        }
    } => Err(e) => assert_vir_error_msg(e, "trusted declarations are not allowed within #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_assume_specification verus_code! {
        #[verifier::no_cheating]
        pub assume_specification<T, U> [Option::<(T, U)>::unzip](a: Option<(T, U)>)
            -> (r: (Option<T>, Option<U>)) ensures false;
    } => Err(e) => assert_vir_error_msg(e, "trusted declarations are not allowed within #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_assume_termination verus_code! {
        #[verifier::no_cheating]
        #[verifier::assume_termination]
        fn checked(i: u64) { checked(i); }
    } => Err(e) => assert_vir_error_msg(e, "#[verifier::assume_termination] not allowed with #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_cannot_disable verus_code! {
        #[verifier::no_cheating]
        proof fn checked() {
            #[verifier::no_cheating(false)]
            proof fn nested() { assume(false); }
        }
    } => Err(e) => assert_vir_error_msg(e, "unrecognized verifier attribute")
}

test_verify_one_file! {
    #[test] scoped_undeclared_external_call verus_code! {
        #[verifier::external]
        fn unchecked() {}
        #[verifier::no_cheating]
        #[verifier::assume(externals_available_without_declaration)]
        fn checked() { unchecked(); }
    } => Err(e) => assert_vir_error_msg(e, "call via externals_available_without_declaration not allowed with #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_unused_macro_helper verus_code! {
        macro_rules! hidden {
            () => {
                #[verifier::proof]
                fn unused() { verus_builtin::assume_(false); }
            }
        }
        #[verifier::no_cheating]
        fn checked() { hidden!(); }
    } => Err(e) => assert_vir_error_msg(e, "assume/admit not allowed with #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_rejects_unsupported_location verus_code! {
        #[verifier::no_cheating]
        mod ignored {}
    } => Err(e) => assert_vir_error_msg(e, "only supported on functions and methods")
}

test_verify_one_file! {
    #[test] scoped_plain_rust_function code! {
        #[verifier::no_cheating]
        fn checked() -> u8 { 7 }
    } => Ok(())
}

test_verify_one_file! {
    #[test] scoped_nested_module_exclusion verus_code! {
        #[verifier::no_cheating]
        fn checked() {
            mod nested {
                #[verifier::external]
                fn hidden() {}
            }
        }
    } => Err(e) => assert_vir_error_msg(e, "trusted declarations are not allowed within #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_nested_trusted_type verus_code! {
        #[verifier::no_cheating]
        fn checked() {
            #[verifier::external_body]
            struct Hidden { field: u8 }
        }
    } => Err(e) => assert_vir_error_msg(e, "trusted declarations are not allowed within #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_trait_method verus_code! {
        trait Protected {
            #[verifier::no_cheating]
            proof fn checked(&self) { assume(false); }
        }
    } => Err(e) => assert_vir_error_msg(e, "assume/admit not allowed with #[verifier::no_cheating]")
}

test_verify_one_file! {
    #[test] scoped_block_annotation_rejected verus_code! {
        proof fn unchecked() {
            #[verifier::no_cheating]
            { assume(false); }
        }
    } => Err(e) => assert_vir_error_msg(e, "only supported on functions and methods")
}

test_verify_one_file_with_options! {
    #[test] test_no_cheating_external_body ["--no-cheating"] => verus_code! {
        #[verifier::external_body] fn no_good() // FAILS
        {}
    } => Err(e) => assert_one_fails(e)
}

test_verify_one_file_with_options! {
    #[test] test_no_cheating_external_body_trait ["--no-cheating"] => verus_code! {
        trait A {
            #[verifier::external_body] fn no_good(&self) // FAILS
            {}
        }
    } => Err(e) => assert_one_fails(e)
}

test_verify_one_file_with_options! {
    #[test] test_no_cheating_external_body_trait_impl ["--no-cheating"] => verus_code! {
        trait A {
            fn no_good(&self) {}
        }

        struct S;

        impl A for S {
            #[verifier::external_body] fn no_good(&self) // FAILS
            {}
        }
    } => Err(e) => assert_one_fails(e)
}

test_verify_one_file_with_options! {
    #[test] test_no_cheating_assume ["--no-cheating"] => verus_code! {
        proof fn test() {
            assume(1 + 1 == 3); // FAILS
        }
    } => Err(e) => assert_one_fails(e)
}

test_verify_one_file_with_options! {
    #[test] test_no_cheating_admit ["--no-cheating"] => verus_code! {
        proof fn test() {
            admit(); // FAILS
        }
    } => Err(e) => assert_one_fails(e)
}

test_verify_one_file_with_options! {
    #[test] test_no_cheating_assume_proof_block ["--no-cheating"] => verus_code! {
        fn test() {
            proof {
                assume(1 + 1 == 3); // FAILS
            }
        }
    } => Err(e) => assert_one_fails(e)
}

test_verify_one_file_with_options! {
    #[test] test_no_cheating_admit_proof_block ["--no-cheating"] => verus_code! {
        fn test() {
            proof {
                admit(); // FAILS
            }
        }
    } => Err(e) => assert_one_fails(e)
}

test_verify_one_file_with_options! {
    #[test] test_no_cheating_assume_spec ["--no-cheating"] => verus_code! {
        use vstd::prelude::*;
        pub assume_specification<T, U> [Option::<(T, U)>::unzip](a: Option<(T, U)>) // FAILS
            -> (r: (Option<T>, Option<U>))
            ensures false
        ;
    } => Err(e) => assert_one_fails(e)
}

test_verify_one_file_with_options! {
    #[test] test_no_cheating_via_fn ["--no-cheating", "--no-verify"] => verus_code! {
        spec fn up(i: int) -> int
            decreases i via no_good
        {
            up(i + 1)
        }

        #[verifier::external_body]
        #[via_fn]
        proof fn no_good(i: int) // FAILS
        {}
    } => Err(e) => assert_one_fails(e)
}

test_verify_one_file_with_options! {
    #[test] test_no_cheating_admit_exec_allows_no_decreases_clause ["--no-cheating"] => verus_code! {
        #[verifier::assume_termination]
        fn a(mut i: u64)
            requires i <= 10,
        {
            a(i)
        }
    } => Err(err) => assert_vir_error_msg(err, "#[verifier::assume_termination] not allowed with --no-cheating")
}
