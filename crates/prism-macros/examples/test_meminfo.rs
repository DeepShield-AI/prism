use prism_macros::ProcParser;
use uom::si::{f64::Information, information::byte};

#[derive(ProcParser, Debug)]
#[fmt = "kv"]
struct MemInfo {
	mem_total: Information,
	mem_free: Information,
	mem_available: Information,
	buffers: Information,
	cached: Information,
	swap_cached: Information,
	active: Information,
	inactive: Information,
	active_anon: Information,
	inactive_anon: Information,
	active_file: Information,
	inactive_file: Information,
	unevictable: Information,
	mlocked: Information,
	swap_total: Information,
	swap_free: Information,
	dirty: Information,
	writeback: Information,
	anon_pages: Information,
	mapped: Information,
	shmem: Information,
	kreclaimable: Information,
	slab: Information,
	slab_reclaimable: Information,
	slab_unreclaimable: Information,
	kernel_stack: Information,
	page_tables: Information,
	nfs_unstable: Information,
	bounce: Information,
	writeback_tmp: Information,
	commit_limit: Information,
	committed_as: Information,
	vmalloc_total: Information,
	vmalloc_used: Information,
	vmalloc_chunk: Information,
	percpu: Information,
	hardware_corrupted: Information,
	anon_huge_pages: Information,
	shmem_huge_pages: Information,
	shmem_pmd_mapped: Information,
	file_huge_pages: Information,
	file_pmd_mapped: Information,
	huge_pages_total: u64,
	huge_pages_free: u64,
	huge_pages_rsvd: u64,
	huge_pages_surp: u64,
	hugepagesize: Information,
	hugetlb: Information,
	direct_map_4k: Information,
	direct_map_2m: Information,
	direct_map_1g: Information,
}

fn main() {
	let sample_meminfo = r#"
MemTotal:       16384000 kB
MemFree:         8192000 kB
MemAvailable:   12288000 kB
Buffers:          512000 kB
Cached:          2048000 kB
SwapCached:           0 kB
Active:          4096000 kB
Inactive:        2048000 kB
Active(anon):    1024000 kB
Inactive(anon):   256000 kB
Active(file):    3072000 kB
Inactive(file):  1792000 kB
Unevictable:           0 kB
Mlocked:               0 kB
SwapTotal:       4194304 kB
SwapFree:        4194304 kB
Dirty:             32768 kB
Writeback:             0 kB
AnonPages:       1048576 kB
Mapped:           524288 kB
Shmem:            131072 kB
KReclaimable:     262144 kB
Slab:             524288 kB
SReclaimable:     262144 kB
SUnreclaim:       262144 kB
KernelStack:       16384 kB
PageTables:        32768 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:    12386304 kB
Committed_AS:    2097152 kB
VmallocTotal:   34359738367 kB
VmallocUsed:       65536 kB
VmallocChunk:          0 kB
Percpu:             8192 kB
HardwareCorrupted:     0 kB
AnonHugePages:    524288 kB
ShmemHugePages:        0 kB
ShmemPmdMapped:        0 kB
FileHugePages:         0 kB
FilePmdMapped:         0 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
Hugetlb:               0 kB
DirectMap4k:      524288 kB
DirectMap2M:    16252928 kB
DirectMap1G:           0 kB
"#;

	match MemInfo::parse(sample_meminfo) {
		Ok(meminfo) => {
			println!("Parsed MemInfo successfully!");
			println!("Total Memory: {} bytes", meminfo.mem_total.get::<byte>());
			println!("Free Memory: {} bytes", meminfo.mem_free.get::<byte>());
			println!("Available Memory: {} bytes", meminfo.mem_available.get::<byte>());
			println!("Active(anon): {} bytes", meminfo.active_anon.get::<byte>());
			println!("Inactive(file): {} bytes", meminfo.inactive_file.get::<byte>());
		},
		Err(e) => {
			eprintln!("Failed to parse meminfo: {}", e);
		},
	}
}
