/// # Glossary (because i don't know where else to put it)
///   IR   - Intermediate Representation  
///   SoN  - Sea Of Nodes (https://www.oracle.com/technetwork/java/javase/tech/c2-ir95-150110.pdf)  
///   SSA  - Single Static Assignment  
///   VN   - Value Number  
///   GVN  - Global Value Numbering  
///   CSE  - Common Subexpression Elimination  
///   CFG  - Control Flow Graph  
///   DSE  - Dead Store Elimination  
///   GCM  - Global Code Motion  
///   SROA - Scalar Replacement Of Aggregates  
///   CCP  - Conditional Constant Propagation  
///   SCCP - Sparse Conditional Constant Propagation  
///   RPO  - Reverse PostOrder  
///   RA   - Register Allocation  
///   BB   - Basic Block  
///   ZTC  - Zero Trip Count  
///   MAF  - Monotone Analysis Framework  
///   SCC  - Strongly Connected Components  
///   MOP  - Meet Over all Paths  
///   IPO  - InterProcedural Optimizations  
///   RPC  - Return Program Counter  
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

