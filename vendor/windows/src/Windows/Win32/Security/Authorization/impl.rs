#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AuthzInterfaceClsid(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetAuthzInterfaceClsid(&self, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Version(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetVersion(&self, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GenerateAudits(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetGenerateAudits(&self, bprop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ApplyStoreSacl(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetApplyStoreSacl(&self, bprop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Writable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn PolicyAdministrators(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn PolicyReaders(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddPolicyAdministrator(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministrator(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReader(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReader(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Scopes(&self) -> ::windows_core::Result<IAzScopes>;
    fn OpenScope(&self, bstrscopename: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzScope>;
    fn CreateScope(&self, bstrscopename: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzScope>;
    fn DeleteScope(&self, bstrscopename: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Operations(&self) -> ::windows_core::Result<IAzOperations>;
    fn OpenOperation(&self, bstroperationname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzOperation>;
    fn CreateOperation(&self, bstroperationname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzOperation>;
    fn DeleteOperation(&self, bstroperationname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Tasks(&self) -> ::windows_core::Result<IAzTasks>;
    fn OpenTask(&self, bstrtaskname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzTask>;
    fn CreateTask(&self, bstrtaskname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzTask>;
    fn DeleteTask(&self, bstrtaskname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(&self, bstrgroupname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(&self, bstrgroupname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&self, bstrgroupname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Roles(&self) -> ::windows_core::Result<IAzRoles>;
    fn OpenRole(&self, bstrrolename: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzRole>;
    fn CreateRole(&self, bstrrolename: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzRole>;
    fn DeleteRole(&self, bstrrolename: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn InitializeClientContextFromToken(&self, ulltokenhandle: u64, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzClientContext>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn InitializeClientContextFromName(&self, clientname: &::windows_core::BSTR, domainname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzClientContext>;
    fn DelegatedPolicyUsers(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn InitializeClientContextFromStringSid(&self, sidstring: &::windows_core::BSTR, loptions: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzClientContext>;
    fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn PolicyReadersName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddPolicyAdministratorName(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministratorName(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReaderName(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReaderName(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DelegatedPolicyUsersName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzApplication {}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>() -> IAzApplication_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn AuthzInterfaceClsid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthzInterfaceClsid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthzInterfaceClsid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthzInterfaceClsid(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVersion(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn GenerateAudits<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateAudits() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGenerateAudits(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn ApplyStoreSacl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplyStoreSacl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplyStoreSacl(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Scopes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscopecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Scopes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscopecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenScope(::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScope<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateScope(::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScope<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteScope(::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Operations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoperationcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperationcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenOperation(::core::mem::transmute(&bstroperationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateOperation(::core::mem::transmute(&bstroperationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteOperation(::core::mem::transmute(&bstroperationname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Tasks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroupcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Roles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Roles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprolecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromToken<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulltokenhandle: u64, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializeClientContextFromToken(::core::mem::transmute_copy(&ulltokenhandle), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientname: ::std::mem::MaybeUninit<::windows_core::BSTR>, domainname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializeClientContextFromName(::core::mem::transmute(&clientname), ::core::mem::transmute(&domainname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DelegatedPolicyUsers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDelegatedPolicyUser(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteDelegatedPolicyUser(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromStringSid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sidstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializeClientContextFromStringSid(::core::mem::transmute(&sidstring), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DelegatedPolicyUsersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDelegatedPolicyUserName(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteDelegatedPolicyUserName(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            AuthzInterfaceClsid: AuthzInterfaceClsid::<Identity, Impl, OFFSET>,
            SetAuthzInterfaceClsid: SetAuthzInterfaceClsid::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            GenerateAudits: GenerateAudits::<Identity, Impl, OFFSET>,
            SetGenerateAudits: SetGenerateAudits::<Identity, Impl, OFFSET>,
            ApplyStoreSacl: ApplyStoreSacl::<Identity, Impl, OFFSET>,
            SetApplyStoreSacl: SetApplyStoreSacl::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, Impl, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, Impl, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, Impl, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, Impl, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, Impl, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, Impl, OFFSET>,
            Scopes: Scopes::<Identity, Impl, OFFSET>,
            OpenScope: OpenScope::<Identity, Impl, OFFSET>,
            CreateScope: CreateScope::<Identity, Impl, OFFSET>,
            DeleteScope: DeleteScope::<Identity, Impl, OFFSET>,
            Operations: Operations::<Identity, Impl, OFFSET>,
            OpenOperation: OpenOperation::<Identity, Impl, OFFSET>,
            CreateOperation: CreateOperation::<Identity, Impl, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            OpenTask: OpenTask::<Identity, Impl, OFFSET>,
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, Impl, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, Impl, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, Impl, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, Impl, OFFSET>,
            Roles: Roles::<Identity, Impl, OFFSET>,
            OpenRole: OpenRole::<Identity, Impl, OFFSET>,
            CreateRole: CreateRole::<Identity, Impl, OFFSET>,
            DeleteRole: DeleteRole::<Identity, Impl, OFFSET>,
            InitializeClientContextFromToken: InitializeClientContextFromToken::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            InitializeClientContextFromName: InitializeClientContextFromName::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsers: DelegatedPolicyUsers::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUser: AddDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUser: DeleteDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            InitializeClientContextFromStringSid: InitializeClientContextFromStringSid::<Identity, Impl, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, Impl, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, Impl, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, Impl, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, Impl, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, Impl, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsersName: DelegatedPolicyUsersName::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUserName: AddDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUserName: DeleteDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzApplication as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplication2_Impl: Sized + IAzApplication_Impl {
    fn InitializeClientContextFromToken2(&self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzClientContext2>;
    fn InitializeClientContext2(&self, identifyingstring: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzClientContext2>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzApplication2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication2_Impl, const OFFSET: isize>() -> IAzApplication2_Vtbl {
        unsafe extern "system" fn InitializeClientContextFromToken2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializeClientContextFromToken2(::core::mem::transmute_copy(&ultokenhandlelowpart), ::core::mem::transmute_copy(&ultokenhandlehighpart), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeClientContext2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifyingstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializeClientContext2(::core::mem::transmute(&identifyingstring), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzApplication_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitializeClientContextFromToken2: InitializeClientContextFromToken2::<Identity, Impl, OFFSET>,
            InitializeClientContext2: InitializeClientContext2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzApplication2 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzApplication as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplication3_Impl: Sized + IAzApplication2_Impl {
    fn ScopeExists(&self, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn OpenScope2(&self, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<IAzScope2>;
    fn CreateScope2(&self, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<IAzScope2>;
    fn DeleteScope2(&self, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleDefinitions(&self) -> ::windows_core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(&self, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(&self, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(&self, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleAssignments(&self) -> ::windows_core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(&self, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(&self, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(&self, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRulesEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetBizRulesEnabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzApplication3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>() -> IAzApplication3_Vtbl {
        unsafe extern "system" fn ScopeExists<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbexist: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScopeExists(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbexist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppscope2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenScope2(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScope2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppscope2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateScope2(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScope2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteScope2(::core::mem::transmute(&bstrscopename)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)).into()
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)).into()
        }
        unsafe extern "system" fn BizRulesEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRulesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRulesEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRulesEnabled(::core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base__: IAzApplication2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ScopeExists: ScopeExists::<Identity, Impl, OFFSET>,
            OpenScope2: OpenScope2::<Identity, Impl, OFFSET>,
            CreateScope2: CreateScope2::<Identity, Impl, OFFSET>,
            DeleteScope2: DeleteScope2::<Identity, Impl, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
            CreateRoleDefinition: CreateRoleDefinition::<Identity, Impl, OFFSET>,
            OpenRoleDefinition: OpenRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
            CreateRoleAssignment: CreateRoleAssignment::<Identity, Impl, OFFSET>,
            OpenRoleAssignment: OpenRoleAssignment::<Identity, Impl, OFFSET>,
            DeleteRoleAssignment: DeleteRoleAssignment::<Identity, Impl, OFFSET>,
            BizRulesEnabled: BizRulesEnabled::<Identity, Impl, OFFSET>,
            SetBizRulesEnabled: SetBizRulesEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzApplication3 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzApplication as ::windows_core::Interface>::IID || iid == &<IAzApplication2 as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Type(&self) -> ::windows_core::Result<i32>;
    fn SetType(&self, lprop: i32) -> ::windows_core::Result<()>;
    fn LdapQuery(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLdapQuery(&self, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AppMembers(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AppNonMembers(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Members(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn NonMembers(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddAppMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteAppMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddAppNonMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteAppNonMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddNonMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteNonMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Writable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddMemberName(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteMemberName(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddNonMemberName(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteNonMemberName(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn MembersName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn NonMembersName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzApplicationGroup {}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>() -> IAzApplicationGroup_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn LdapQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LdapQuery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLdapQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLdapQuery(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn AppMembers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppMembers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppNonMembers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppNonMembers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Members<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Members() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonMembers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NonMembers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn AddAppMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAppMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAppMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddAppNonMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAppNonMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppNonMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAppNonMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddNonMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNonMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteNonMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteNonMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddMemberName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMemberName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddNonMemberName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNonMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteNonMemberName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteNonMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn MembersName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MembersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonMembersName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NonMembersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            LdapQuery: LdapQuery::<Identity, Impl, OFFSET>,
            SetLdapQuery: SetLdapQuery::<Identity, Impl, OFFSET>,
            AppMembers: AppMembers::<Identity, Impl, OFFSET>,
            AppNonMembers: AppNonMembers::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            NonMembers: NonMembers::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            AddAppMember: AddAppMember::<Identity, Impl, OFFSET>,
            DeleteAppMember: DeleteAppMember::<Identity, Impl, OFFSET>,
            AddAppNonMember: AddAppNonMember::<Identity, Impl, OFFSET>,
            DeleteAppNonMember: DeleteAppNonMember::<Identity, Impl, OFFSET>,
            AddMember: AddMember::<Identity, Impl, OFFSET>,
            DeleteMember: DeleteMember::<Identity, Impl, OFFSET>,
            AddNonMember: AddNonMember::<Identity, Impl, OFFSET>,
            DeleteNonMember: DeleteNonMember::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            AddMemberName: AddMemberName::<Identity, Impl, OFFSET>,
            DeleteMemberName: DeleteMemberName::<Identity, Impl, OFFSET>,
            AddNonMemberName: AddNonMemberName::<Identity, Impl, OFFSET>,
            DeleteNonMemberName: DeleteNonMemberName::<Identity, Impl, OFFSET>,
            MembersName: MembersName::<Identity, Impl, OFFSET>,
            NonMembersName: NonMembersName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzApplicationGroup as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationGroup2_Impl: Sized + IAzApplicationGroup_Impl {
    fn BizRule(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRule(&self, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRuleLanguage(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRuleLanguage(&self, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRuleImportedPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRuleImportedPath(&self, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleAssignments(&self, bstrscopename: &::windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IAzRoleAssignments>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzApplicationGroup2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroup2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>() -> IAzApplicationGroup2_Vtbl {
        unsafe extern "system" fn BizRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRule(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRuleLanguage(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleImportedPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleImportedPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRuleImportedPath(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments(::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzApplicationGroup_Vtbl::new::<Identity, Impl, OFFSET>(),
            BizRule: BizRule::<Identity, Impl, OFFSET>,
            SetBizRule: SetBizRule::<Identity, Impl, OFFSET>,
            BizRuleLanguage: BizRuleLanguage::<Identity, Impl, OFFSET>,
            SetBizRuleLanguage: SetBizRuleLanguage::<Identity, Impl, OFFSET>,
            BizRuleImportedPath: BizRuleImportedPath::<Identity, Impl, OFFSET>,
            SetBizRuleImportedPath: SetBizRuleImportedPath::<Identity, Impl, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzApplicationGroup2 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzApplicationGroup as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationGroups_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzApplicationGroups {}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroups_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>() -> IAzApplicationGroups_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzApplicationGroups as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplications_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzApplications {}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplications_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: isize>() -> IAzApplications_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzApplications as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzAuthorizationStore_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DomainTimeout(&self) -> ::windows_core::Result<i32>;
    fn SetDomainTimeout(&self, lprop: i32) -> ::windows_core::Result<()>;
    fn ScriptEngineTimeout(&self) -> ::windows_core::Result<i32>;
    fn SetScriptEngineTimeout(&self, lprop: i32) -> ::windows_core::Result<()>;
    fn MaxScriptEngines(&self) -> ::windows_core::Result<i32>;
    fn SetMaxScriptEngines(&self, lprop: i32) -> ::windows_core::Result<()>;
    fn GenerateAudits(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetGenerateAudits(&self, bprop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Writable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddPropertyItem(&self, lpropid: AZ_PROP_CONSTANTS, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn PolicyAdministrators(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn PolicyReaders(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddPolicyAdministrator(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministrator(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReader(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReader(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Initialize(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn UpdateCache(&self, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Delete(&self, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Applications(&self) -> ::windows_core::Result<IAzApplications>;
    fn OpenApplication(&self, bstrapplicationname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzApplication>;
    fn CreateApplication(&self, bstrapplicationname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzApplication>;
    fn DeleteApplication(&self, bstrapplicationname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups>;
    fn CreateApplicationGroup(&self, bstrgroupname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn OpenApplicationGroup(&self, bstrgroupname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&self, bstrgroupname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DelegatedPolicyUsers(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn TargetMachine(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ApplyStoreSacl(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetApplyStoreSacl(&self, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn PolicyReadersName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddPolicyAdministratorName(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministratorName(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReaderName(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReaderName(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DelegatedPolicyUsersName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn CloseApplication(&self, bstrapplicationname: &::windows_core::BSTR, lflag: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzAuthorizationStore {}
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>() -> IAzAuthorizationStore_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn DomainTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DomainTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDomainTimeout(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn ScriptEngineTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScriptEngineTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScriptEngineTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScriptEngineTimeout(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn MaxScriptEngines<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxScriptEngines() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxScriptEngines<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxScriptEngines(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn GenerateAudits<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateAudits() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGenerateAudits(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: AZ_PROP_CONSTANTS, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrpolicyurl), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn UpdateCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateCache(::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Applications<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Applications() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenApplication(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplication(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteApplication(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroupcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DelegatedPolicyUsers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDelegatedPolicyUser(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteDelegatedPolicyUser(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn TargetMachine<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetmachine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TargetMachine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtargetmachine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyStoreSacl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbapplystoresacl: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplyStoreSacl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbapplystoresacl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplyStoreSacl(::core::mem::transmute_copy(&bapplystoresacl)).into()
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DelegatedPolicyUsersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDelegatedPolicyUserName(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteDelegatedPolicyUserName(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn CloseApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflag: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseApplication(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute_copy(&lflag)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            DomainTimeout: DomainTimeout::<Identity, Impl, OFFSET>,
            SetDomainTimeout: SetDomainTimeout::<Identity, Impl, OFFSET>,
            ScriptEngineTimeout: ScriptEngineTimeout::<Identity, Impl, OFFSET>,
            SetScriptEngineTimeout: SetScriptEngineTimeout::<Identity, Impl, OFFSET>,
            MaxScriptEngines: MaxScriptEngines::<Identity, Impl, OFFSET>,
            SetMaxScriptEngines: SetMaxScriptEngines::<Identity, Impl, OFFSET>,
            GenerateAudits: GenerateAudits::<Identity, Impl, OFFSET>,
            SetGenerateAudits: SetGenerateAudits::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, Impl, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, Impl, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, Impl, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, Impl, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, Impl, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            UpdateCache: UpdateCache::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Applications: Applications::<Identity, Impl, OFFSET>,
            OpenApplication: OpenApplication::<Identity, Impl, OFFSET>,
            CreateApplication: CreateApplication::<Identity, Impl, OFFSET>,
            DeleteApplication: DeleteApplication::<Identity, Impl, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, Impl, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, Impl, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, Impl, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsers: DelegatedPolicyUsers::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUser: AddDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUser: DeleteDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            TargetMachine: TargetMachine::<Identity, Impl, OFFSET>,
            ApplyStoreSacl: ApplyStoreSacl::<Identity, Impl, OFFSET>,
            SetApplyStoreSacl: SetApplyStoreSacl::<Identity, Impl, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, Impl, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, Impl, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, Impl, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, Impl, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, Impl, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsersName: DelegatedPolicyUsersName::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUserName: AddDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUserName: DeleteDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
            CloseApplication: CloseApplication::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzAuthorizationStore as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzAuthorizationStore2_Impl: Sized + IAzAuthorizationStore_Impl {
    fn OpenApplication2(&self, bstrapplicationname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzApplication2>;
    fn CreateApplication2(&self, bstrapplicationname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzApplication2>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzAuthorizationStore2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>() -> IAzAuthorizationStore2_Vtbl {
        unsafe extern "system" fn OpenApplication2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenApplication2(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplication2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplication2(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzAuthorizationStore_Vtbl::new::<Identity, Impl, OFFSET>(),
            OpenApplication2: OpenApplication2::<Identity, Impl, OFFSET>,
            CreateApplication2: CreateApplication2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzAuthorizationStore2 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzAuthorizationStore as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzAuthorizationStore3_Impl: Sized + IAzAuthorizationStore2_Impl {
    fn IsUpdateNeeded(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BizruleGroupSupported(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn UpgradeStoresFunctionalLevel(&self, lfunctionallevel: i32) -> ::windows_core::Result<()>;
    fn IsFunctionalLevelUpgradeSupported(&self, lfunctionallevel: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSchemaVersion(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzAuthorizationStore3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>() -> IAzAuthorizationStore3_Vtbl {
        unsafe extern "system" fn IsUpdateNeeded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisupdateneeded: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUpdateNeeded() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisupdateneeded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizruleGroupSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizruleGroupSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpgradeStoresFunctionalLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpgradeStoresFunctionalLevel(::core::mem::transmute_copy(&lfunctionallevel)).into()
        }
        unsafe extern "system" fn IsFunctionalLevelUpgradeSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32, pbsupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFunctionalLevelUpgradeSupported(::core::mem::transmute_copy(&lfunctionallevel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemaVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSchemaVersion(::core::mem::transmute_copy(&plmajorversion), ::core::mem::transmute_copy(&plminorversion)).into()
        }
        Self {
            base__: IAzAuthorizationStore2_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsUpdateNeeded: IsUpdateNeeded::<Identity, Impl, OFFSET>,
            BizruleGroupSupported: BizruleGroupSupported::<Identity, Impl, OFFSET>,
            UpgradeStoresFunctionalLevel: UpgradeStoresFunctionalLevel::<Identity, Impl, OFFSET>,
            IsFunctionalLevelUpgradeSupported: IsFunctionalLevelUpgradeSupported::<Identity, Impl, OFFSET>,
            GetSchemaVersion: GetSchemaVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzAuthorizationStore3 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzAuthorizationStore as ::windows_core::Interface>::IID || iid == &<IAzAuthorizationStore2 as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzBizRuleContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetBusinessRuleResult(&self, bresult: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetBusinessRuleString(&self, bstrbusinessrulestring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BusinessRuleString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetParameter(&self, bstrparametername: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzBizRuleContext {}
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>() -> IAzBizRuleContext_Vtbl {
        unsafe extern "system" fn SetBusinessRuleResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bresult: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBusinessRuleResult(::core::mem::transmute_copy(&bresult)).into()
        }
        unsafe extern "system" fn SetBusinessRuleString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbusinessrulestring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBusinessRuleString(::core::mem::transmute(&bstrbusinessrulestring)).into()
        }
        unsafe extern "system" fn BusinessRuleString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BusinessRuleString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbusinessrulestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarparametervalue: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParameter(::core::mem::transmute(&bstrparametername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarparametervalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBusinessRuleResult: SetBusinessRuleResult::<Identity, Impl, OFFSET>,
            SetBusinessRuleString: SetBusinessRuleString::<Identity, Impl, OFFSET>,
            BusinessRuleString: BusinessRuleString::<Identity, Impl, OFFSET>,
            GetParameter: GetParameter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzBizRuleContext as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzBizRuleInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddInterface(&self, bstrinterfacename: &::windows_core::BSTR, linterfaceflag: i32, varinterface: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddInterfaces(&self, varinterfacenames: &::windows_core::VARIANT, varinterfaceflags: &::windows_core::VARIANT, varinterfaces: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn GetInterfaceValue(&self, bstrinterfacename: &::windows_core::BSTR, linterfaceflag: *mut i32, varinterface: *mut ::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Remove(&self, bstrinterfacename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveAll(&self) -> ::windows_core::Result<()>;
    fn Count(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzBizRuleInterfaces {}
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleInterfaces_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>() -> IAzBizRuleInterfaces_Vtbl {
        unsafe extern "system" fn AddInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::std::mem::MaybeUninit<::windows_core::BSTR>, linterfaceflag: i32, varinterface: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddInterface(::core::mem::transmute(&bstrinterfacename), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute(&varinterface)).into()
        }
        unsafe extern "system" fn AddInterfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinterfacenames: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varinterfaceflags: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varinterfaces: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddInterfaces(::core::mem::transmute(&varinterfacenames), ::core::mem::transmute(&varinterfaceflags), ::core::mem::transmute(&varinterfaces)).into()
        }
        unsafe extern "system" fn GetInterfaceValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::std::mem::MaybeUninit<::windows_core::BSTR>, linterfaceflag: *mut i32, varinterface: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInterfaceValue(::core::mem::transmute(&bstrinterfacename), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute_copy(&varinterface)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&bstrinterfacename)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAll().into()
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddInterface: AddInterface::<Identity, Impl, OFFSET>,
            AddInterfaces: AddInterfaces::<Identity, Impl, OFFSET>,
            GetInterfaceValue: GetInterfaceValue::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzBizRuleInterfaces as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzBizRuleParameters_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddParameter(&self, bstrparametername: &::windows_core::BSTR, varparametervalue: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddParameters(&self, varparameternames: &::windows_core::VARIANT, varparametervalues: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn GetParameterValue(&self, bstrparametername: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Remove(&self, varparametername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveAll(&self) -> ::windows_core::Result<()>;
    fn Count(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzBizRuleParameters {}
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleParameters_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>() -> IAzBizRuleParameters_Vtbl {
        unsafe extern "system" fn AddParameter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::std::mem::MaybeUninit<::windows_core::BSTR>, varparametervalue: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddParameter(::core::mem::transmute(&bstrparametername), ::core::mem::transmute(&varparametervalue)).into()
        }
        unsafe extern "system" fn AddParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varparameternames: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varparametervalues: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddParameters(::core::mem::transmute(&varparameternames), ::core::mem::transmute(&varparametervalues)).into()
        }
        unsafe extern "system" fn GetParameterValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarparametervalue: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParameterValue(::core::mem::transmute(&bstrparametername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarparametervalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varparametername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&varparametername)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAll().into()
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddParameter: AddParameter::<Identity, Impl, OFFSET>,
            AddParameters: AddParameters::<Identity, Impl, OFFSET>,
            GetParameterValue: GetParameterValue::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzBizRuleParameters as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzClientContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AccessCheck(&self, bstrobjectname: &::windows_core::BSTR, varscopenames: &::windows_core::VARIANT, varoperations: &::windows_core::VARIANT, varparameternames: &::windows_core::VARIANT, varparametervalues: &::windows_core::VARIANT, varinterfacenames: &::windows_core::VARIANT, varinterfaceflags: &::windows_core::VARIANT, varinterfaces: &::windows_core::VARIANT) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn GetBusinessRuleString(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserDn(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserSamCompat(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserDisplay(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserGuid(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserCanonical(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserUpn(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserDnsSamCompat(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetProperty(&self, lpropid: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn GetRoles(&self, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn RoleForAccessCheck(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRoleForAccessCheck(&self, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzClientContext {}
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>() -> IAzClientContext_Vtbl {
        unsafe extern "system" fn AccessCheck<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>,
            varscopenames: ::std::mem::MaybeUninit<::windows_core::VARIANT>,
            varoperations: ::std::mem::MaybeUninit<::windows_core::VARIANT>,
            varparameternames: ::std::mem::MaybeUninit<::windows_core::VARIANT>,
            varparametervalues: ::std::mem::MaybeUninit<::windows_core::VARIANT>,
            varinterfacenames: ::std::mem::MaybeUninit<::windows_core::VARIANT>,
            varinterfaceflags: ::std::mem::MaybeUninit<::windows_core::VARIANT>,
            varinterfaces: ::std::mem::MaybeUninit<::windows_core::VARIANT>,
            pvarresults: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>,
        ) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccessCheck(::core::mem::transmute(&bstrobjectname), ::core::mem::transmute(&varscopenames), ::core::mem::transmute(&varoperations), ::core::mem::transmute(&varparameternames), ::core::mem::transmute(&varparametervalues), ::core::mem::transmute(&varinterfacenames), ::core::mem::transmute(&varinterfaceflags), ::core::mem::transmute(&varinterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBusinessRuleString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBusinessRuleString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbusinessrulestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserDn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSamCompat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserSamCompat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDisplay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserCanonical<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserCanonical() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserUpn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserUpn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDnsSamCompat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserDnsSamCompat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarrolenames: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRoles(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarrolenames, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleForAccessCheck<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleForAccessCheck() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoleForAccessCheck<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRoleForAccessCheck(::core::mem::transmute(&bstrprop)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AccessCheck: AccessCheck::<Identity, Impl, OFFSET>,
            GetBusinessRuleString: GetBusinessRuleString::<Identity, Impl, OFFSET>,
            UserDn: UserDn::<Identity, Impl, OFFSET>,
            UserSamCompat: UserSamCompat::<Identity, Impl, OFFSET>,
            UserDisplay: UserDisplay::<Identity, Impl, OFFSET>,
            UserGuid: UserGuid::<Identity, Impl, OFFSET>,
            UserCanonical: UserCanonical::<Identity, Impl, OFFSET>,
            UserUpn: UserUpn::<Identity, Impl, OFFSET>,
            UserDnsSamCompat: UserDnsSamCompat::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRoles: GetRoles::<Identity, Impl, OFFSET>,
            RoleForAccessCheck: RoleForAccessCheck::<Identity, Impl, OFFSET>,
            SetRoleForAccessCheck: SetRoleForAccessCheck::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzClientContext as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzClientContext2_Impl: Sized + IAzClientContext_Impl {
    fn GetAssignedScopesPage(&self, loptions: i32, pagesize: i32, pvarcursor: *mut ::windows_core::VARIANT, pvarscopenames: *mut ::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddRoles(&self, varroles: &::windows_core::VARIANT, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddApplicationGroups(&self, varapplicationgroups: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddStringSids(&self, varstringsids: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn SetLDAPQueryDN(&self, bstrldapquerydn: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LDAPQueryDN(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzClientContext2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>() -> IAzClientContext2_Vtbl {
        unsafe extern "system" fn GetAssignedScopesPage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32, pagesize: i32, pvarcursor: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvarscopenames: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAssignedScopesPage(::core::mem::transmute_copy(&loptions), ::core::mem::transmute_copy(&pagesize), ::core::mem::transmute_copy(&pvarcursor), ::core::mem::transmute_copy(&pvarscopenames)).into()
        }
        unsafe extern "system" fn AddRoles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varroles: ::std::mem::MaybeUninit<::windows_core::VARIANT>, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRoles(::core::mem::transmute(&varroles), ::core::mem::transmute(&bstrscopename)).into()
        }
        unsafe extern "system" fn AddApplicationGroups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varapplicationgroups: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddApplicationGroups(::core::mem::transmute(&varapplicationgroups)).into()
        }
        unsafe extern "system" fn AddStringSids<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstringsids: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStringSids(::core::mem::transmute(&varstringsids)).into()
        }
        unsafe extern "system" fn SetLDAPQueryDN<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrldapquerydn: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLDAPQueryDN(::core::mem::transmute(&bstrldapquerydn)).into()
        }
        unsafe extern "system" fn LDAPQueryDN<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrldapquerydn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LDAPQueryDN() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrldapquerydn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzClientContext_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAssignedScopesPage: GetAssignedScopesPage::<Identity, Impl, OFFSET>,
            AddRoles: AddRoles::<Identity, Impl, OFFSET>,
            AddApplicationGroups: AddApplicationGroups::<Identity, Impl, OFFSET>,
            AddStringSids: AddStringSids::<Identity, Impl, OFFSET>,
            SetLDAPQueryDN: SetLDAPQueryDN::<Identity, Impl, OFFSET>,
            LDAPQueryDN: LDAPQueryDN::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzClientContext2 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzClientContext as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzClientContext3_Impl: Sized + IAzClientContext2_Impl {
    fn AccessCheck2(&self, bstrobjectname: &::windows_core::BSTR, bstrscopename: &::windows_core::BSTR, loperation: i32) -> ::windows_core::Result<u32>;
    fn IsInRoleAssignment(&self, bstrscopename: &::windows_core::BSTR, bstrrolename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetOperations(&self, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<IAzOperations>;
    fn GetTasks(&self, bstrscopename: &::windows_core::BSTR) -> ::windows_core::Result<IAzTasks>;
    fn BizRuleParameters(&self) -> ::windows_core::Result<IAzBizRuleParameters>;
    fn BizRuleInterfaces(&self) -> ::windows_core::Result<IAzBizRuleInterfaces>;
    fn GetGroups(&self, bstrscopename: &::windows_core::BSTR, uloptions: &AZ_PROP_CONSTANTS) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Sids(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzClientContext3 {}
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>() -> IAzClientContext3_Vtbl {
        unsafe extern "system" fn AccessCheck2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, loperation: i32, plresult: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccessCheck2(::core::mem::transmute(&bstrobjectname), ::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&loperation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInRoleAssignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbisinrole: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInRoleAssignment(::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&bstrrolename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisinrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppoperationcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOperations(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperationcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTasks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTasks(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbizruleparam: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbizruleparam, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleInterfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbizruleinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbizruleinterfaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, uloptions: u32, pgrouparray: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGroups(::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&uloptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrouparray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sids<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstringsidarray: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Sids() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstringsidarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzClientContext2_Vtbl::new::<Identity, Impl, OFFSET>(),
            AccessCheck2: AccessCheck2::<Identity, Impl, OFFSET>,
            IsInRoleAssignment: IsInRoleAssignment::<Identity, Impl, OFFSET>,
            GetOperations: GetOperations::<Identity, Impl, OFFSET>,
            GetTasks: GetTasks::<Identity, Impl, OFFSET>,
            BizRuleParameters: BizRuleParameters::<Identity, Impl, OFFSET>,
            BizRuleInterfaces: BizRuleInterfaces::<Identity, Impl, OFFSET>,
            GetGroups: GetGroups::<Identity, Impl, OFFSET>,
            Sids: Sids::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzClientContext3 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzClientContext as ::windows_core::Interface>::IID || iid == &<IAzClientContext2 as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzNameResolver_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn NameFromSid(&self, bstrsid: &::windows_core::BSTR, psidtype: *mut i32, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NamesFromSids(&self, vsids: &::windows_core::VARIANT, pvsidtypes: *mut ::windows_core::VARIANT, pvnames: *mut ::windows_core::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzNameResolver {}
#[cfg(feature = "Win32_System_Com")]
impl IAzNameResolver_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzNameResolver_Impl, const OFFSET: isize>() -> IAzNameResolver_Vtbl {
        unsafe extern "system" fn NameFromSid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzNameResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsid: ::std::mem::MaybeUninit<::windows_core::BSTR>, psidtype: *mut i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NameFromSid(::core::mem::transmute(&bstrsid), ::core::mem::transmute_copy(&psidtype), ::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn NamesFromSids<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzNameResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vsids: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvsidtypes: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvnames: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NamesFromSids(::core::mem::transmute(&vsids), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            NameFromSid: NameFromSid::<Identity, Impl, OFFSET>,
            NamesFromSids: NamesFromSids::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzNameResolver as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzObjectPicker_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetPrincipals(&self, hparentwnd: super::super::Foundation::HWND, bstrtitle: &::windows_core::BSTR, pvsidtypes: *mut ::windows_core::VARIANT, pvnames: *mut ::windows_core::VARIANT, pvsids: *mut ::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzObjectPicker {}
#[cfg(feature = "Win32_System_Com")]
impl IAzObjectPicker_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzObjectPicker_Impl, const OFFSET: isize>() -> IAzObjectPicker_Vtbl {
        unsafe extern "system" fn GetPrincipals<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND, bstrtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvsidtypes: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvnames: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvsids: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrincipals(::core::mem::transmute_copy(&hparentwnd), ::core::mem::transmute(&bstrtitle), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames), ::core::mem::transmute_copy(&pvsids)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPrincipals: GetPrincipals::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzObjectPicker as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzOperation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OperationID(&self) -> ::windows_core::Result<i32>;
    fn SetOperationID(&self, lprop: i32) -> ::windows_core::Result<()>;
    fn Writable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzOperation {}
#[cfg(feature = "Win32_System_Com")]
impl IAzOperation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>() -> IAzOperation_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn OperationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OperationID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOperationID(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            OperationID: OperationID::<Identity, Impl, OFFSET>,
            SetOperationID: SetOperationID::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzOperation as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzOperation2_Impl: Sized + IAzOperation_Impl {
    fn RoleAssignments(&self, bstrscopename: &::windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IAzRoleAssignments>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzOperation2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAzOperation2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation2_Impl, const OFFSET: isize>() -> IAzOperation2_Vtbl {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments(::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IAzOperation_Vtbl::new::<Identity, Impl, OFFSET>(), RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzOperation2 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzOperation as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzOperations_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzOperations {}
#[cfg(feature = "Win32_System_Com")]
impl IAzOperations_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: isize>() -> IAzOperations_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzOperations as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzPrincipalLocator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn NameResolver(&self) -> ::windows_core::Result<IAzNameResolver>;
    fn ObjectPicker(&self) -> ::windows_core::Result<IAzObjectPicker>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzPrincipalLocator {}
#[cfg(feature = "Win32_System_Com")]
impl IAzPrincipalLocator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>() -> IAzPrincipalLocator_Vtbl {
        unsafe extern "system" fn NameResolver<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnameresolver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NameResolver() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnameresolver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectPicker<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjectpicker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ObjectPicker() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjectpicker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            NameResolver: NameResolver::<Identity, Impl, OFFSET>,
            ObjectPicker: ObjectPicker::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzPrincipalLocator as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRole_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddAppMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteAppMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddTask(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteTask(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddOperation(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteOperation(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteMember(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Writable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AppMembers(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Members(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Operations(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Tasks(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddMemberName(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteMemberName(&self, bstrprop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn MembersName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzRole {}
#[cfg(feature = "Win32_System_Com")]
impl IAzRole_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>() -> IAzRole_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn AddAppMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAppMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAppMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTask(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTask(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOperation(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteOperation(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AppMembers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppMembers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Members<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Members() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Operations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddMemberName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMemberName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn MembersName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MembersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            AddAppMember: AddAppMember::<Identity, Impl, OFFSET>,
            DeleteAppMember: DeleteAppMember::<Identity, Impl, OFFSET>,
            AddTask: AddTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            AddOperation: AddOperation::<Identity, Impl, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, Impl, OFFSET>,
            AddMember: AddMember::<Identity, Impl, OFFSET>,
            DeleteMember: DeleteMember::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AppMembers: AppMembers::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            Operations: Operations::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            AddMemberName: AddMemberName::<Identity, Impl, OFFSET>,
            DeleteMemberName: DeleteMemberName::<Identity, Impl, OFFSET>,
            MembersName: MembersName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzRole as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleAssignment_Impl: Sized + IAzRole_Impl {
    fn AddRoleDefinition(&self, bstrroledefinition: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DeleteRoleDefinition(&self, bstrroledefinition: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleDefinitions(&self) -> ::windows_core::Result<IAzRoleDefinitions>;
    fn Scope(&self) -> ::windows_core::Result<IAzScope>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzRoleAssignment {}
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleAssignment_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>() -> IAzRoleAssignment_Vtbl {
        unsafe extern "system" fn AddRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRoleDefinition(::core::mem::transmute(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleDefinition(::core::mem::transmute(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Scope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzRole_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddRoleDefinition: AddRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzRoleAssignment as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzRole as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleAssignments_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzRoleAssignments {}
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleAssignments_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>() -> IAzRoleAssignments_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzRoleAssignments as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleDefinition_Impl: Sized + IAzTask_Impl {
    fn RoleAssignments(&self, bstrscopename: &::windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IAzRoleAssignments>;
    fn AddRoleDefinition(&self, bstrroledefinition: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DeleteRoleDefinition(&self, bstrroledefinition: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleDefinitions(&self) -> ::windows_core::Result<IAzRoleDefinitions>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzRoleDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleDefinition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>() -> IAzRoleDefinition_Vtbl {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments(::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRoleDefinition(::core::mem::transmute(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleDefinition(::core::mem::transmute(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzTask_Vtbl::new::<Identity, Impl, OFFSET>(),
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
            AddRoleDefinition: AddRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzRoleDefinition as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzTask as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleDefinitions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzRoleDefinitions {}
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleDefinitions_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>() -> IAzRoleDefinitions_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzRoleDefinitions as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoles_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzRoles {}
#[cfg(feature = "Win32_System_Com")]
impl IAzRoles_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: isize>() -> IAzRoles_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzRoles as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzScope_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Writable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn PolicyAdministrators(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn PolicyReaders(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddPolicyAdministrator(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministrator(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReader(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReader(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(&self, bstrgroupname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(&self, bstrgroupname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&self, bstrgroupname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Roles(&self) -> ::windows_core::Result<IAzRoles>;
    fn OpenRole(&self, bstrrolename: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzRole>;
    fn CreateRole(&self, bstrrolename: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzRole>;
    fn DeleteRole(&self, bstrrolename: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Tasks(&self) -> ::windows_core::Result<IAzTasks>;
    fn OpenTask(&self, bstrtaskname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzTask>;
    fn CreateTask(&self, bstrtaskname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<IAzTask>;
    fn DeleteTask(&self, bstrtaskname: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn CanBeDelegated(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn BizrulesWritable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn PolicyReadersName(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddPolicyAdministratorName(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyAdministratorName(&self, bstradmin: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddPolicyReaderName(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePolicyReaderName(&self, bstrreader: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzScope {}
#[cfg(feature = "Win32_System_Com")]
impl IAzScope_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>() -> IAzScope_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroupcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Roles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Roles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprolecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Tasks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn CanBeDelegated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanBeDelegated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizrulesWritable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizrulesWritable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, Impl, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, Impl, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, Impl, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, Impl, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, Impl, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, Impl, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, Impl, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, Impl, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, Impl, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, Impl, OFFSET>,
            Roles: Roles::<Identity, Impl, OFFSET>,
            OpenRole: OpenRole::<Identity, Impl, OFFSET>,
            CreateRole: CreateRole::<Identity, Impl, OFFSET>,
            DeleteRole: DeleteRole::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            OpenTask: OpenTask::<Identity, Impl, OFFSET>,
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            CanBeDelegated: CanBeDelegated::<Identity, Impl, OFFSET>,
            BizrulesWritable: BizrulesWritable::<Identity, Impl, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, Impl, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, Impl, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, Impl, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, Impl, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, Impl, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzScope as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzScope2_Impl: Sized + IAzScope_Impl {
    fn RoleDefinitions(&self) -> ::windows_core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(&self, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(&self, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(&self, bstrroledefinitionname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RoleAssignments(&self) -> ::windows_core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(&self, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(&self, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(&self, bstrroleassignmentname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzScope2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAzScope2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>() -> IAzScope2_Vtbl {
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)).into()
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)).into()
        }
        Self {
            base__: IAzScope_Vtbl::new::<Identity, Impl, OFFSET>(),
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
            CreateRoleDefinition: CreateRoleDefinition::<Identity, Impl, OFFSET>,
            OpenRoleDefinition: OpenRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
            CreateRoleAssignment: CreateRoleAssignment::<Identity, Impl, OFFSET>,
            OpenRoleAssignment: OpenRoleAssignment::<Identity, Impl, OFFSET>,
            DeleteRoleAssignment: DeleteRoleAssignment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzScope2 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzScope as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzScopes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzScopes {}
#[cfg(feature = "Win32_System_Com")]
impl IAzScopes_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: isize>() -> IAzScopes_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzScopes as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzTask_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ApplicationData(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRule(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRule(&self, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRuleLanguage(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRuleLanguage(&self, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BizRuleImportedPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBizRuleImportedPath(&self, bstrprop: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsRoleDefinition(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsRoleDefinition(&self, fprop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Operations(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Tasks(&self) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn AddOperation(&self, bstrop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteOperation(&self, bstrop: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddTask(&self, bstrtask: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteTask(&self, bstrtask: &::windows_core::BSTR, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Writable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &::windows_core::VARIANT, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &::windows_core::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzTask {}
#[cfg(feature = "Win32_System_Com")]
impl IAzTask_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>() -> IAzTask_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn BizRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRule(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRuleLanguage(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleImportedPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleImportedPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRuleImportedPath(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn IsRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRoleDefinition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRoleDefinition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fprop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsRoleDefinition(::core::mem::transmute_copy(&fprop)).into()
        }
        unsafe extern "system" fn Operations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOperation(::core::mem::transmute(&bstrop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrop: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteOperation(::core::mem::transmute(&bstrop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtask: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTask(::core::mem::transmute(&bstrtask), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtask: ::std::mem::MaybeUninit<::windows_core::BSTR>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTask(::core::mem::transmute(&bstrtask), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>, pvarprop: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::std::mem::MaybeUninit<::windows_core::VARIANT>, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            BizRule: BizRule::<Identity, Impl, OFFSET>,
            SetBizRule: SetBizRule::<Identity, Impl, OFFSET>,
            BizRuleLanguage: BizRuleLanguage::<Identity, Impl, OFFSET>,
            SetBizRuleLanguage: SetBizRuleLanguage::<Identity, Impl, OFFSET>,
            BizRuleImportedPath: BizRuleImportedPath::<Identity, Impl, OFFSET>,
            SetBizRuleImportedPath: SetBizRuleImportedPath::<Identity, Impl, OFFSET>,
            IsRoleDefinition: IsRoleDefinition::<Identity, Impl, OFFSET>,
            SetIsRoleDefinition: SetIsRoleDefinition::<Identity, Impl, OFFSET>,
            Operations: Operations::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            AddOperation: AddOperation::<Identity, Impl, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, Impl, OFFSET>,
            AddTask: AddTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzTask as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzTask2_Impl: Sized + IAzTask_Impl {
    fn RoleAssignments(&self, bstrscopename: &::windows_core::BSTR, brecursive: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IAzRoleAssignments>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzTask2 {}
#[cfg(feature = "Win32_System_Com")]
impl IAzTask2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask2_Impl, const OFFSET: isize>() -> IAzTask2_Vtbl {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::std::mem::MaybeUninit<::windows_core::BSTR>, brecursive: super::super::Foundation::VARIANT_BOOL, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments(::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IAzTask_Vtbl::new::<Identity, Impl, OFFSET>(), RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzTask2 as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID || iid == &<IAzTask as ::windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzTasks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::VARIANT>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IAzTasks {}
#[cfg(feature = "Win32_System_Com")]
impl IAzTasks_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: isize>() -> IAzTasks_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAzTasks as ::windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::Interface>::IID
    }
}
