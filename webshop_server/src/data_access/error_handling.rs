#[derive(Debug)]
pub enum PostgresDBError {
    SuccessfulCompletion,
    Warning,
    DynamicResultSetsReturned,
    ImplicitZeroBitPadding,
    NullValueEliminatedInSetFunction,
    PrivilegeNotGranted,
    PrivilegeNotRevoked,
    StringDataRightTruncation,
    DeprecatedFeature,
    NoData,
    NoAdditionalDynamicResultSetsReturned,
    SqlStatementNotYetComplete,
    ConnectionException,
    ConnectionDoesNotExist,
    ConnectionFailure,
    SqlclientUnableToEstablishSqlconnection,
    SqlserverRejectedEstablishmentOfSqlconnection,
    TransactionResolutionUnknown,
    ProtocolViolation,
    TriggeredActionException,
    FeatureNotSupported,
    InvalidTransactionInitiation,
    LocatorException,
    InvalidLocatorSpecification,
    InvalidGrantor,
    InvalidGrantOperation,
    InvalidRoleSpecification,
    DiagnosticsException,
    StackedDiagnosticsAccessedWithoutActiveHandler,
    CaseNotFound,
    CardinalityViolation,
    DataException,
    ArraySubscriptError,
    CharacterNotInRepertoire,
    DatetimeFieldOverflow,
    DivisionByZero,
    ErrorInAssignment,
    EscapeCharacterConflict,
    IndicatorOverflow,
    IntervalFieldOverflow,
    InvalidArgumentForLogarithm,
    InvalidArgumentForNtileFunction,
    InvalidArgumentForNthValueFunction,
    InvalidArgumentForPowerFunction,
    InvalidArgumentForWidthBucketFunction,
    InvalidCharacterValueForCast,
    InvalidDatetimeFormat,
    InvalidEscapeCharacter,
    InvalidEscapeOctet,
    InvalidEscapeSequence,
    NonstandardUseOfEscapeCharacter,
    InvalidIndicatorParameterValue,
    InvalidParameterValue,
    InvalidPrecedingOrFollowingSize,
    InvalidRegularExpression,
    InvalidRowCountInLimitClause,
    InvalidRowCountInResultOffsetClause,
    InvalidTablesampleArgument,
    InvalidTablesampleRepeat,
    InvalidTimeZoneDisplacementValue,
    InvalidUseOfEscapeCharacter,
    MostSpecificTypeMismatch,
    NullValueNotAllowed,
    NullValueNoIndicatorParameter,
    NumericValueOutOfRange,
    SequenceGeneratorLimitExceeded,
    StringDataLengthMismatch,
    SubstringError,
    TrimError,
    UnterminatedCString,
    ZeroLengthCharacterString,
    FloatingPointException,
    InvalidTextRepresentation,
    InvalidBinaryRepresentation,
    BadCopyFileFormat,
    UntranslatableCharacter,
    NotAnXmlDocument,
    InvalidXmlDocument,
    InvalidXmlContent,
    InvalidXmlComment,
    InvalidXmlProcessingInstruction,
    DuplicateJsonObjectKeyValue,
    InvalidArgumentForSqlJsonDatetimeFunction,
    InvalidJsonText,
    InvalidSqlJsonSubscript,
    MoreThanOneSqlJsonItem,
    NoSqlJsonItem,
    NonNumericSqlJsonItem,
    NonUniqueKeysInAJsonObject,
    SingletonSqlJsonItemRequired,
    SqlJsonArrayNotFound,
    SqlJsonMemberNotFound,
    SqlJsonNumberNotFound,
    SqlJsonObjectNotFound,
    TooManyJsonArrayElements,
    TooManyJsonObjectMembers,
    SqlJsonScalarRequired,
    SqlJsonItemCannotBeCastToTargetType,
    IntegrityConstraintViolation,
    RestrictViolation,
    NotNullViolation,
    ForeignKeyViolation,
    UniqueViolation,
    CheckViolation,
    ExclusionViolation,
    InvalidCursorState,
    InvalidTransactionState,
    ActiveSqlTransaction,
    BranchTransactionAlreadyActive,
    HeldCursorRequiresSameIsolationLevel,
    InappropriateAccessModeForBranchTransaction,
    InappropriateIsolationLevelForBranchTransaction,
    NoActiveSqlTransactionForBranchTransaction,
    ReadOnlySqlTransaction,
    SchemaAndDataStatementMixingNotSupported,
    NoActiveSqlTransaction,
    InFailedSqlTransaction,
    IdleInTransactionSessionTimeout,
    InvalidSqlStatementName,
    TriggeredDataChangeViolation,
    InvalidAuthorizationSpecification,
    InvalidPassword,
    DependentPrivilegeDescriptorsStillExist,
    DependentObjectsStillExist,
    InvalidTransactionTermination,
    SqlRoutineException,
    FunctionExecutedNoReturnStatement,
    ModifyingSqlDataNotPermitted,
    ProhibitedSqlStatementAttempted,
    ReadingSqlDataNotPermitted,
    InvalidCursorName,
    ExternalRoutineException,
    ContainingSqlNotPermitted,
    ExternalRoutineInvocationException,
    InvalidSqlstateReturned,
    TriggerProtocolViolated,
    SrfProtocolViolated,
    EventTriggerProtocolViolated,
    SavepointException,
    InvalidSavepointSpecification,
    InvalidCatalogName,
    InvalidSchemaName,
    TransactionRollback,
    TransactionIntegrityConstraintViolation,
    SerializationFailure,
    StatementCompletionUnknown,
    DeadlockDetected,
    SyntaxErrorOrAccessRuleViolation,
    SyntaxError,
    InsufficientPrivilege,
    CannotCoerce,
    GroupingError,
    WindowingError,
    InvalidRecursion,
    InvalidForeignKey,
    InvalidName,
    NameTooLong,
    ReservedName,
    DatatypeMismatch,
    IndeterminateDatatype,
    CollationMismatch,
    IndeterminateCollation,
    WrongObjectType,
    GeneratedAlways,
    UndefinedColumn,
    UndefinedFunction,
    UndefinedTable,
    UndefinedParameter,
    UndefinedObject,
    DuplicateColumn,
    DuplicateCursor,
    DuplicateDatabase,
    DuplicateFunction,
    DuplicatePreparedStatement,
    DuplicateSchema,
    DuplicateTable,
    DuplicateAlias,
    DuplicateObject,
    AmbiguousColumn,
    AmbiguousFunction,
    AmbiguousParameter,
    AmbiguousAlias,
    InvalidColumnReference,
    InvalidColumnDefinition,
    InvalidCursorDefinition,
    InvalidDatabaseDefinition,
    InvalidFunctionDefinition,
    InvalidPreparedStatementDefinition,
    InvalidSchemaDefinition,
    InvalidTableDefinition,
    InvalidObjectDefinition,
    WithCheckOptionViolation,
    InsufficientResources,
    DiskFull,
    OutOfMemory,
    TooManyConnections,
    ConfigurationLimitExceeded,
    ProgramLimitExceeded,
    StatementTooComplex,
    TooManyColumns,
    TooManyArguments,
    ObjectNotInPrerequisiteState,
    ObjectInUse,
    CantChangeRuntimeParam,
    LockNotAvailable,
    UnsafeNewEnumValueUsage,
    OperatorIntervention,
    QueryCanceled,
    AdminShutdown,
    CrashShutdown,
    CannotConnectNow,
    DatabaseDropped,
    IdleSessionTimeout,
    SystemError,
    IoError,
    UndefinedFile,
    DuplicateFile,
    SnapshotTooOld,
    ConfigFileError,
    LockFileExists,
    FdwError,
    FdwColumnNameNotFound,
    FdwDynamicParameterValueNeeded,
    FdwFunctionSequenceError,
    FdwInconsistentDescriptorInformation,
    FdwInvalidAttributeValue,
    FdwInvalidColumnName,
    FdwInvalidColumnNumber,
    FdwInvalidDataType,
    FdwInvalidDataTypeDescriptors,
    FdwInvalidDescriptorFieldIdentifier,
    FdwInvalidHandle,
    FdwInvalidOptionIndex,
    FdwInvalidOptionName,
    FdwInvalidStringLengthOrBufferLength,
    FdwInvalidStringFormat,
    FdwInvalidUseOfNullPointer,
    FdwTooManyHandles,
    FdwOutOfMemory,
    FdwNoSchemas,
    FdwOptionNameNotFound,
    FdwReplyHandle,
    FdwSchemaNotFound,
    FdwTableNotFound,
    FdwUnableToCreateExecution,
    FdwUnableToCreateReply,
    FdwUnableToEstablishConnection,
    PlpgsqlError,
    RaiseException,
    NoDataFound,
    TooManyRows,
    AssertFailure,
    InternalError,
    DataCorrupted,
    IndexCorrupted,

    NoErrorCode,
}

impl PostgresDBError {
    /// Method to take error code from postgres and turn it into an readable error
    /// Link to error codes: https://www.postgresql.org/docs/current/errcodes-appendix.html
    /// # Arguments
    /// * `error` - The error code from postgres
    /// # Returns
    /// * `Self` - The error code as a readable error as a enum variant from PostgresDBError
    /// # Example
    /// ```rust
    /// async fn companies(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    ///     let companies = company::get_all_companies(&pool).await;
    ///     match companies {
    ///         Ok(companies) => HttpResponse::Ok().json(companies),
    ///         Err(e) => match e {
    ///             sqlx::Error::Database(e) => match error_handling::PostgresDBError::from_str(e) {
    ///                 error_handling::PostgresDBError::NoDataFound => HttpResponse::NotFound().json("No companies found"),
    ///                 // if the data already exists, return a Conflict
    ///                 error_handling::PostgresDBError::UniqueViolation => HttpResponse::Conflict().json("Company already exists"),
    ///                 _ => HttpResponse::InternalServerError().json("Internal Server Error"),
    ///             },
    ///             _ => HttpResponse::InternalServerError().json("Internal Server Error"),
    ///         },
    ///     }
    /// }
    /// ```
    pub fn from_str(error: Box<dyn sqlx::error::DatabaseError>) -> Self {
        match error.code() {
            Some(e) => match e.as_ref() {
                "00000" => Self::SuccessfulCompletion,
                "01000" => Self::Warning,
                "0100C" => Self::DynamicResultSetsReturned,
                "01008" => Self::ImplicitZeroBitPadding,
                "01003" => Self::NullValueEliminatedInSetFunction,
                "01007" => Self::PrivilegeNotGranted,
                "01006" => Self::PrivilegeNotRevoked,
                "01004" => Self::StringDataRightTruncation,
                "01P01" => Self::DeprecatedFeature,
                "02000" => Self::NoData,
                "02001" => Self::NoAdditionalDynamicResultSetsReturned,
                "03000" => Self::SqlStatementNotYetComplete,
                "08000" => Self::ConnectionException,
                "08003" => Self::ConnectionDoesNotExist,
                "08006" => Self::ConnectionFailure,
                "08001" => Self::SqlclientUnableToEstablishSqlconnection,
                "08004" => Self::SqlserverRejectedEstablishmentOfSqlconnection,
                "08007" => Self::TransactionResolutionUnknown,
                "08P01" => Self::ProtocolViolation,
                "09000" => Self::TriggeredActionException,
                "0A000" => Self::FeatureNotSupported,
                "0B000" => Self::InvalidTransactionInitiation,
                "0F000" => Self::LocatorException,
                "0F001" => Self::InvalidLocatorSpecification,
                "0L000" => Self::InvalidGrantor,
                "0LP01" => Self::InvalidGrantOperation,
                "0P000" => Self::InvalidRoleSpecification,
                "0Z000" => Self::DiagnosticsException,
                "0Z002" => Self::StackedDiagnosticsAccessedWithoutActiveHandler,
                "20000" => Self::CaseNotFound,
                "21000" => Self::CardinalityViolation,
                "22000" => Self::DataException,
                "2202E" => Self::ArraySubscriptError,
                "22021" => Self::CharacterNotInRepertoire,
                "22008" => Self::DatetimeFieldOverflow,
                "22012" => Self::DivisionByZero,
                "22005" => Self::ErrorInAssignment,
                "2200B" => Self::EscapeCharacterConflict,
                "22022" => Self::IndicatorOverflow,
                "22015" => Self::IntervalFieldOverflow,
                "2201E" => Self::InvalidArgumentForLogarithm,
                "22014" => Self::InvalidArgumentForNtileFunction,
                "22016" => Self::InvalidArgumentForNthValueFunction,
                "2201F" => Self::InvalidArgumentForPowerFunction,
                "2201G" => Self::InvalidArgumentForWidthBucketFunction,
                "22018" => Self::InvalidCharacterValueForCast,
                "22007" => Self::InvalidDatetimeFormat,
                "22019" => Self::InvalidEscapeCharacter,
                "2200D" => Self::InvalidEscapeOctet,
                "22025" => Self::InvalidEscapeSequence,
                "22P06" => Self::NonstandardUseOfEscapeCharacter,
                "22010" => Self::InvalidIndicatorParameterValue,
                "22023" => Self::InvalidParameterValue,
                "22013" => Self::InvalidPrecedingOrFollowingSize,
                "2201B" => Self::InvalidRegularExpression,
                "2201W" => Self::InvalidRowCountInLimitClause,
                "2201X" => Self::InvalidRowCountInResultOffsetClause,
                "2202H" => Self::InvalidTablesampleArgument,
                "2202G" => Self::InvalidTablesampleRepeat,
                "22009" => Self::InvalidTimeZoneDisplacementValue,
                "2200C" => Self::InvalidUseOfEscapeCharacter,
                "2200G" => Self::MostSpecificTypeMismatch,
                "22004" => Self::NullValueNotAllowed,
                "22002" => Self::NullValueNoIndicatorParameter,
                "22003" => Self::NumericValueOutOfRange,
                "2200H" => Self::SequenceGeneratorLimitExceeded,
                "22026" => Self::StringDataLengthMismatch,
                "22001" => Self::StringDataRightTruncation,
                "22011" => Self::SubstringError,
                "22027" => Self::TrimError,
                "22024" => Self::UnterminatedCString,
                "2200F" => Self::ZeroLengthCharacterString,
                "22P01" => Self::FloatingPointException,
                "22P02" => Self::InvalidTextRepresentation,
                "22P03" => Self::InvalidBinaryRepresentation,
                "22P04" => Self::BadCopyFileFormat,
                "22P05" => Self::UntranslatableCharacter,
                "2200L" => Self::NotAnXmlDocument,
                "2200M" => Self::InvalidXmlDocument,
                "2200N" => Self::InvalidXmlContent,
                "2200S" => Self::InvalidXmlComment,
                "2200T" => Self::InvalidXmlProcessingInstruction,
                "22030" => Self::DuplicateJsonObjectKeyValue,
                "22031" => Self::InvalidArgumentForSqlJsonDatetimeFunction,
                "22032" => Self::InvalidJsonText,
                "22033" => Self::InvalidSqlJsonSubscript,
                "22034" => Self::MoreThanOneSqlJsonItem,
                "22035" => Self::NoSqlJsonItem,
                "22036" => Self::NonNumericSqlJsonItem,
                "22037" => Self::NonUniqueKeysInAJsonObject,
                "22038" => Self::SingletonSqlJsonItemRequired,
                "22039" => Self::SqlJsonArrayNotFound,
                "2203A" => Self::SqlJsonMemberNotFound,
                "2203B" => Self::SqlJsonNumberNotFound,
                "2203C" => Self::SqlJsonObjectNotFound,
                "2203D" => Self::TooManyJsonArrayElements,
                "2203E" => Self::TooManyJsonObjectMembers,
                "2203F" => Self::SqlJsonScalarRequired,
                "2203G" => Self::SqlJsonItemCannotBeCastToTargetType,
                "23000" => Self::IntegrityConstraintViolation,
                "23001" => Self::RestrictViolation,
                "23502" => Self::NotNullViolation,
                "23503" => Self::ForeignKeyViolation,
                "23505" => Self::UniqueViolation,
                "23514" => Self::CheckViolation,
                "23P01" => Self::ExclusionViolation,
                "24000" => Self::InvalidCursorState,
                "25000" => Self::InvalidTransactionState,
                "25001" => Self::ActiveSqlTransaction,
                "25002" => Self::BranchTransactionAlreadyActive,
                "25008" => Self::HeldCursorRequiresSameIsolationLevel,
                "25003" => Self::InappropriateAccessModeForBranchTransaction,
                "25004" => Self::InappropriateIsolationLevelForBranchTransaction,
                "25005" => Self::NoActiveSqlTransactionForBranchTransaction,
                "25006" => Self::ReadOnlySqlTransaction,
                "25007" => Self::SchemaAndDataStatementMixingNotSupported,
                "25P01" => Self::NoActiveSqlTransaction,
                "25P02" => Self::InFailedSqlTransaction,
                "25P03" => Self::IdleInTransactionSessionTimeout,
                "26000" => Self::InvalidSqlStatementName,
                "27000" => Self::TriggeredDataChangeViolation,
                "28000" => Self::InvalidAuthorizationSpecification,
                "28P01" => Self::InvalidPassword,
                "2B000" => Self::DependentPrivilegeDescriptorsStillExist,
                "2BP01" => Self::DependentObjectsStillExist,
                "2D000" => Self::InvalidTransactionTermination,
                "2F000" => Self::SqlRoutineException,
                "2F005" => Self::FunctionExecutedNoReturnStatement,
                "2F002" => Self::ModifyingSqlDataNotPermitted,
                "2F003" => Self::ProhibitedSqlStatementAttempted,
                "2F004" => Self::ReadingSqlDataNotPermitted,
                "34000" => Self::InvalidCursorName,
                "38000" => Self::ExternalRoutineException,
                "38001" => Self::ContainingSqlNotPermitted,
                "38002" => Self::ModifyingSqlDataNotPermitted,
                "38003" => Self::ProhibitedSqlStatementAttempted,
                "38004" => Self::ReadingSqlDataNotPermitted,
                "39000" => Self::ExternalRoutineInvocationException,
                "39001" => Self::InvalidSqlstateReturned,
                "39004" => Self::NullValueNotAllowed,
                "39P01" => Self::TriggerProtocolViolated,
                "39P02" => Self::SrfProtocolViolated,
                "39P03" => Self::EventTriggerProtocolViolated,
                "3B000" => Self::SavepointException,
                "3B001" => Self::InvalidSavepointSpecification,
                "3D000" => Self::InvalidCatalogName,
                "3F000" => Self::InvalidSchemaName,
                "40000" => Self::TransactionRollback,
                "40002" => Self::TransactionIntegrityConstraintViolation,
                "40001" => Self::SerializationFailure,
                "40003" => Self::StatementCompletionUnknown,
                "40P01" => Self::DeadlockDetected,
                "42000" => Self::SyntaxErrorOrAccessRuleViolation,
                "42601" => Self::SyntaxError,
                "42501" => Self::InsufficientPrivilege,
                "42846" => Self::CannotCoerce,
                "42803" => Self::GroupingError,
                "42P20" => Self::WindowingError,
                "42P19" => Self::InvalidRecursion,
                "42830" => Self::InvalidForeignKey,
                "42602" => Self::InvalidName,
                "42622" => Self::NameTooLong,
                "42939" => Self::ReservedName,
                "42804" => Self::DatatypeMismatch,
                "42P18" => Self::IndeterminateDatatype,
                "42P21" => Self::CollationMismatch,
                "42P22" => Self::IndeterminateCollation,
                "42809" => Self::WrongObjectType,
                "428C9" => Self::GeneratedAlways,
                "42703" => Self::UndefinedColumn,
                "42883" => Self::UndefinedFunction,
                "42P01" => Self::UndefinedTable,
                "42P02" => Self::UndefinedParameter,
                "42704" => Self::UndefinedObject,
                "42701" => Self::DuplicateColumn,
                "42P03" => Self::DuplicateCursor,
                "42P04" => Self::DuplicateDatabase,
                "42723" => Self::DuplicateFunction,
                "42P05" => Self::DuplicatePreparedStatement,
                "42P06" => Self::DuplicateSchema,
                "42P07" => Self::DuplicateTable,
                "42712" => Self::DuplicateAlias,
                "42710" => Self::DuplicateObject,
                "42702" => Self::AmbiguousColumn,
                "42725" => Self::AmbiguousFunction,
                "42P08" => Self::AmbiguousParameter,
                "42P09" => Self::AmbiguousAlias,
                "42P10" => Self::InvalidColumnReference,
                "42611" => Self::InvalidColumnDefinition,
                "42P11" => Self::InvalidCursorDefinition,
                "42P12" => Self::InvalidDatabaseDefinition,
                "42P13" => Self::InvalidFunctionDefinition,
                "42P14" => Self::InvalidPreparedStatementDefinition,
                "42P15" => Self::InvalidSchemaDefinition,
                "42P16" => Self::InvalidTableDefinition,
                "42P17" => Self::InvalidObjectDefinition,
                "44000" => Self::WithCheckOptionViolation,
                "53000" => Self::InsufficientResources,
                "53100" => Self::DiskFull,
                "53200" => Self::OutOfMemory,
                "53300" => Self::TooManyConnections,
                "53400" => Self::ConfigurationLimitExceeded,
                "54000" => Self::ProgramLimitExceeded,
                "54001" => Self::StatementTooComplex,
                "54011" => Self::TooManyColumns,
                "54023" => Self::TooManyArguments,
                "55000" => Self::ObjectNotInPrerequisiteState,
                "55006" => Self::ObjectInUse,
                "55P02" => Self::CantChangeRuntimeParam,
                "55P03" => Self::LockNotAvailable,
                "55P04" => Self::UnsafeNewEnumValueUsage,
                "57000" => Self::OperatorIntervention,
                "57014" => Self::QueryCanceled,
                "57P01" => Self::AdminShutdown,
                "57P02" => Self::CrashShutdown,
                "57P03" => Self::CannotConnectNow,
                "57P04" => Self::DatabaseDropped,
                "57P05" => Self::IdleSessionTimeout,
                "58000" => Self::SystemError,
                "58030" => Self::IoError,
                "58P01" => Self::UndefinedFile,
                "58P02" => Self::DuplicateFile,
                "72000" => Self::SnapshotTooOld,
                "F0000" => Self::ConfigFileError,
                "F0001" => Self::LockFileExists,
                "HV000" => Self::FdwError,
                "HV005" => Self::FdwColumnNameNotFound,
                "HV002" => Self::FdwDynamicParameterValueNeeded,
                "HV010" => Self::FdwFunctionSequenceError,
                "HV021" => Self::FdwInconsistentDescriptorInformation,
                "HV024" => Self::FdwInvalidAttributeValue,
                "HV007" => Self::FdwInvalidColumnName,
                "HV008" => Self::FdwInvalidColumnNumber,
                "HV004" => Self::FdwInvalidDataType,
                "HV006" => Self::FdwInvalidDataTypeDescriptors,
                "HV091" => Self::FdwInvalidDescriptorFieldIdentifier,
                "HV00B" => Self::FdwInvalidHandle,
                "HV00C" => Self::FdwInvalidOptionIndex,
                "HV00D" => Self::FdwInvalidOptionName,
                "HV090" => Self::FdwInvalidStringLengthOrBufferLength,
                "HV00A" => Self::FdwInvalidStringFormat,
                "HV009" => Self::FdwInvalidUseOfNullPointer,
                "HV014" => Self::FdwTooManyHandles,
                "HV001" => Self::FdwOutOfMemory,
                "HV00P" => Self::FdwNoSchemas,
                "HV00J" => Self::FdwOptionNameNotFound,
                "HV00K" => Self::FdwReplyHandle,
                "HV00Q" => Self::FdwSchemaNotFound,
                "HV00R" => Self::FdwTableNotFound,
                "HV00L" => Self::FdwUnableToCreateExecution,
                "HV00M" => Self::FdwUnableToCreateReply,
                "HV00N" => Self::FdwUnableToEstablishConnection,
                "P0000" => Self::PlpgsqlError,
                "P0001" => Self::RaiseException,
                "P0002" => Self::NoDataFound,
                "P0003" => Self::TooManyRows,
                "P0004" => Self::AssertFailure,
                "XX000" => Self::InternalError,
                "XX001" => Self::DataCorrupted,
                "XX002" => Self::IndexCorrupted,

                _ => Self::NoErrorCode,
            },
            None => Self::NoErrorCode,
        }
    }
}