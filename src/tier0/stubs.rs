#![allow(non_snake_case)]

#[no_mangle]
extern "C" fn Print_ClearClientTime() {}

#[no_mangle]
extern "C" fn Print_EnableTimePrefix() {}

#[no_mangle]
extern "C" fn Print_SetClientTime() {}

#[export_name = "?JTGuts_AddDependentJobArray@@YA?AW4JobID_t@@W4JobTypeID_t@@W41@_K2IIPEBW41@2E@Z"]
extern "C" fn JTGuts_AddDependentJobArray() {}

#[no_mangle]
extern "C" fn LoggingSystem_Log() {}

#[export_name = "LoggingSystem_RegisterLoggingChannel"]
extern "C" fn LoggingSystem_RegisterLoggingChannel() {}

#[no_mangle]
extern "C" fn SendRemoteErrorReport() {}

#[no_mangle]
extern "C" fn StackToolsNotify_LoadedLibrary() {}

#[no_mangle]
extern "C" fn Error() {}

#[no_mangle]
extern "C" fn GetCPUInformation() {}

#[no_mangle]
extern "C" fn MemFreeScratch() {}

#[no_mangle]
extern "C" fn MemAllocScratch() {}

#[no_mangle]
extern "C" fn Print_GetTimeStamp() {}

#[no_mangle]
extern "C" fn ThreadSleep() {}

#[no_mangle]
extern "C" fn ThreadInMainThread() {}

#[no_mangle]
extern "C" fn g_ClockSpeedSecondsMultiplier() {}

#[no_mangle]
extern "C" fn Plat_MSTime() {}

#[no_mangle]
extern "C" fn g_ClockSpeedMillisecondsMultiplier() {}

#[no_mangle]
extern "C" fn ThreadInMainOrServerFrameThread() {}

#[no_mangle]
extern "C" fn vtune() {}

#[no_mangle]
extern "C" fn CoreMsgV() {}

#[no_mangle]
extern "C" fn Plat_ExitProcess() {}

#[no_mangle]
extern "C" fn PlatFloatTime() {}

#[no_mangle]
extern "C" fn Print_ClearServerTime() {}

#[no_mangle]
extern "C" fn Print_SetServerTime() {}

#[no_mangle]
extern "C" fn ThreadInServerFrameThread() {}

#[no_mangle]
extern "C" fn Plat_IsInDebugSession() {}

#[export_name = "?JT_HelpWithJobTypesOrSleep@@YAXP6A_N_K@Z00@Z"]
extern "C" fn JT_HelpWithJobTypesOrSleep() {}

#[export_name = "?JT_ContinueJob@@YAXW4JobTypeID_t@@@Z"]
extern "C" fn JT_ContinueJob() {}

#[export_name = "?JT_EndJobGroup@@YAXW4JobID_t@@@Z"]
extern "C" fn JT_EndJobGroup() {}

#[export_name = "?JT_BeginJobGroup@@YA?AW4JobID_t@@W41@@Z"]
extern "C" fn JT_BeginJobGroup() {}

#[export_name = "?JT_RegisterCallJobType@@YA?AW4JobTypeID_t@@W4JobPriority_e@@PEBD@Z"]
extern "C" fn JT_RegisterCallJobType() {}

#[export_name = "??1CThreadMutex@@QEAA@XZ"]
extern "C" fn CThreadMutex() {}

#[export_name = "??0CThreadMutex@@QEAA@XZ"]
extern "C" fn CThreadMutex0() {}

#[export_name = "?job_JT_ExecuteCall@@3PAW4JobTypeID_t@@A"]
extern "C" fn job_JT_ExecuteCall() {}

#[export_name = "?JT_AddCallArray@@YA?AW4JobID_t@@W4JobCallPriority_e@@W41@P6AX_K@Z222@Z"]
extern "C" fn JT_AddCallArray() {}

#[export_name = "?JTGuts_AddJobArray@@YA?AW4JobID_t@@W4JobTypeID_t@@W41@_K22E@Z"]
extern "C" fn JTGuts_AddJobArray() {}

#[export_name = "?JT_WaitForJobAndOnlyHelpWithJobTypes@@YAXW4JobID_t@@_K1@Z"]
extern "C" fn JT_WaitForJobAndOnlyHelpWithJobTypes() {}

#[export_name = "?JT_IsJobDone@@YA_NW4JobID_t@@@Z"]
extern "C" fn JT_IsJobDone() {}

#[export_name = "?JTGuts_AddJob@@YA?AW4JobID_t@@W4JobTypeID_t@@W41@_K2@Z"]
extern "C" fn JTGuts_AddJob() {}

#[export_name = "?JT_SetJobTypeAffinity@@YAXW4JobTypeID_t@@II@Z"]
extern "C" fn JT_SetJobTypeAffinity() {}

#[export_name = "?JT_InitJobTypeForJobArrays@@YAXW4JobTypeID_t@@_KM@Z"]
extern "C" fn JT_InitJobTypeForJobArrays() {}

#[export_name = "?JTGuts_RegisterJobType@@YA?AW4JobTypeID_t@@W4JobPriority_e@@P6AX_K11@ZP6AXXZ31PEBD@Z"]
extern "C" fn JTGuts_RegisterJobType() {}

#[export_name = "?JT_GetCurrentJob@@YA?AW4JobID_t@@XZ"]
extern "C" fn JT_GetCurrentJob() {}

#[export_name = "?JT_ThreadIndexEnd@@YAIXZ"]
extern "C" fn JT_ThreadIndexEnd() {}

#[export_name = "?g_nThreadID@@3V?$CThreadLocalInt@H@GenericThreadLocals@@A"]
extern "C" fn g_nThreadID() {}

#[export_name = "?SpinLockForRead@CThreadSpinRWLock@@QEAAXXZ"]
extern "C" fn SpinLockForRead() {}

#[export_name = "?SpinLockForWrite@CThreadSpinRWLock@@QEAAXXZ"]
extern "C" fn SpinLockForWrite() {}

#[export_name = "?Get@CThreadLocalBase@GenericThreadLocals@@QEBAPEAXXZ"]
extern "C" fn Get_ThreadLocalBase_GenericThreadLocals() {}

#[export_name = "?g_pakLoadApi@@3UPakLoadFuncs_s@@A"]
extern "C" fn g_pakLoadApi() {}

#[export_name = "?JT_GetCurrentThread@@YAIXZ"]
extern "C" fn JT_GetCurrentThread() {}

#[export_name = "?LockForRead@CThreadSpinRWLock@@QEBAXXZ"]
extern "C" fn LockForRead() {}

#[export_name = "DetectLanguage"]
extern "C" fn DetectLanguage() {}

#[export_name = "RunTSListTests"]
extern "C" fn RunTSListTests() {}

#[export_name = "RunTSQueueTests"]
extern "C" fn RunTSQueueTests() {}

#[export_name = "?Resume@CThread@@QEAAIXZ"]
extern "C" fn Resume_CThread() {}

#[export_name = "?Suspend@CThread@@QEAAIXZ"]
extern "C" fn Suspend_CThread() {}

#[export_name = "?SetPriority@CThread@@QEAA_NW4ThreadPriority_e@@@Z"]
extern "C" fn SetPriority_CThread() {}

#[export_name = "?GetPriority@CThread@@QEBA?AW4ThreadPriority_e@@XZ"]
extern "C" fn GetPriority_CThread_0() {}

#[export_name = "?GetThreadHandle@CThread@@QEAAPEAUV_ThreadHandle_t__@@XZ"]
extern "C" fn GetThreadHandle_CThread() {}

#[export_name = "?IsAlive@CThread@@QEAA_NXZ"]
extern "C" fn IsAlive() {}

#[export_name = "?SetName@CThread@@QEAAXPEBD@Z"]
extern "C" fn SetName() {}

#[export_name = "??1CThread@@UEAA@XZ"]
extern "C" fn CThread_0() {}

#[export_name = "??0CThread@@QEAA@XZ"]
extern "C" fn CThread_1() {}

#[export_name = "?WaitForMultiple@CThreadEvent@@SAIHPEAPEAV1@_NI@Z"]
extern "C" fn WaitForMultiple_CThreadEvent() {}

#[export_name = "?Wait@CThreadEvent@@QEAA_NI@Z"]
extern "C" fn Wait_CThreadEvent() {}

#[export_name = "?Check@CThreadEvent@@QEAA_NXZ"]
extern "C" fn Check_CThreadEvent() {}

#[export_name = "?Reset@CThreadEvent@@QEAA_NXZ"]
extern "C" fn Reset_CThreadEvent() {}

#[export_name = "?Set@CThreadEvent@@QEAA_NXZ"]
extern "C" fn Set_CThreadEvent() {}

#[export_name = "??0CThreadEvent@@QEAA@_N@Z"]
extern "C" fn CThreadEvent() {}

#[export_name = "??1CThreadSyncObject@@QEAA@XZ"]
extern "C" fn CThreadSyncObject() {}

#[export_name = "ThreadSetAffinity"]
extern "C" fn ThreadSetAffinity() {}

#[export_name = "ThreadSetDebugName"]
extern "C" fn ThreadSetDebugName() {}

#[export_name = "ThreadJoin"]
extern "C" fn ThreadJoin() {}

// #[no_mangle]
// extern "C" fn ThreadInMainThread() {}

#[no_mangle]
extern "C" fn ThreadSetPriority() {}

#[no_mangle]
extern "C" fn ThreadGetPriority() {}

#[no_mangle]
extern "C" fn ThreadGetCurrentHandle() {}

#[export_name = "?Set@CThreadLocalBase@GenericThreadLocals@@QEAAXPEAX@Z"]
extern "C" fn Set_CThreadLocalBaseGenericThreadLocal() {}

#[export_name = "??1CThreadLocalBase@GenericThreadLocals@@QEAA@XZ"]
extern "C" fn CThreadLocalBaseGeneric_GenericThreadLocals() {}

#[export_name = "??0CThreadLocalBase@GenericThreadLocals@@QEAA@XZ"]
extern "C" fn CThreadLocalBaseGeneric_GenericThreadLocals_0() {}

#[export_name = "?JTGuts_AddDependentJob@@YA?AW4JobID_t@@W4JobTypeID_t@@W41@_K2IIPEBW41@@Z"]
extern "C" fn JTGuts_AddDependentJob() {}

#[no_mangle]
extern "C" fn Plat_FloatTime() {}

#[export_name = "?JT_GrowJobArray_Lock@@YAIW4JobID_t@@@Z"]
extern "C" fn JT_GrowJobArray_Lock() {}

#[export_name = "?JT_GrowJobArray_Unlock@@YAXW4JobID_t@@I@Z"]
extern "C" fn JT_GrowJobArray_Unlock() {}

#[export_name = "?JT_DoneGrowingJobArray@@YAXW4JobID_t@@@Z"]
extern "C" fn JT_DoneGrowingJobArray() {}

#[export_name = "?JT_GrowJobArray_SingleGrower@@YAXW4JobID_t@@I@Z"]
extern "C" fn JT_GrowJobArray_SingleGrower() {}

#[export_name = "?JT_HelpWithJobTypes@@YA_NP6A_N_K@Z00@Z"]
extern "C" fn JT_HelpWithJobTypes() {}

#[export_name = "?JTGuts_AddJob_Try@@YA?AW4JobID_t@@W4JobTypeID_t@@W41@_K2@Z"]
extern "C" fn JTGuts_AddJob_Try() {}

#[export_name = "?JT_SetThreadIdx@@YAXI@Z"]
extern "C" fn JT_SetThreadIdx() {}

#[export_name = "?Start@CThread@@UEAA_N_KW4ThreadPriorityEnum_t@1@@Z"]
extern "C" fn Start_CThread() {}

#[export_name = "?Run@CThread@@MEAAHXZ"]
extern "C" fn Run_CThread(){}