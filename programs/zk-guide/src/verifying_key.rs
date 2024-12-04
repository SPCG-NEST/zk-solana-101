use groth16_solana::groth16::Groth16Verifyingkey;

pub const VERIFYINGKEY: Groth16Verifyingkey =  Groth16Verifyingkey {
	nr_pubinputs: 2,

	vk_alpha_g1: [
		43,128,31,155,241,168,0,210,45,148,0,236,124,185,155,165,110,41,163,113,28,195,241,25,220,237,91,72,197,220,25,45,
		6,189,124,198,67,80,36,196,234,107,220,130,111,49,38,220,246,109,134,162,100,91,186,140,123,87,158,139,8,38,181,54,
	],

	vk_beta_g2: [
		37,28,22,175,90,109,13,154,254,216,225,208,59,76,85,231,33,3,249,217,22,168,138,204,234,75,151,232,136,107,103,198,
		27,104,189,72,190,44,100,104,120,191,52,140,52,49,160,164,249,149,125,163,30,34,10,245,161,64,102,88,24,140,28,204,
		31,59,232,126,91,194,132,171,232,189,242,233,62,157,67,36,3,163,5,103,7,97,153,76,57,58,2,140,147,50,218,209,
		34,53,7,16,112,188,108,12,220,104,91,20,179,64,187,29,7,182,18,247,246,86,199,4,170,209,18,193,24,245,235,188,
	],

	vk_gamme_g2: [
		25,142,147,147,146,13,72,58,114,96,191,183,49,251,93,37,241,170,73,51,53,169,231,18,151,228,133,183,174,243,18,194,
		24,0,222,239,18,31,30,118,66,106,0,102,94,92,68,121,103,67,34,212,247,94,218,221,70,222,189,92,217,146,246,237,
		9,6,137,208,88,95,240,117,236,158,153,173,105,12,51,149,188,75,49,51,112,179,142,243,85,172,218,220,209,34,151,91,
		18,200,94,165,219,140,109,235,74,171,113,128,141,203,64,143,227,209,231,105,12,67,211,123,76,230,204,1,102,250,125,170,
	],

	vk_delta_g2: [
		0,119,214,234,130,10,78,46,137,200,74,25,53,146,124,199,203,177,65,35,221,197,216,12,138,136,133,40,147,250,107,103,
		8,177,130,34,145,78,156,80,229,114,118,198,180,198,72,35,84,217,30,173,117,97,29,57,132,250,11,245,18,116,212,25,
		13,217,135,19,47,162,209,99,69,47,96,235,139,63,187,145,102,39,84,122,188,76,93,194,163,126,160,160,182,114,136,75,
		15,212,239,198,10,114,121,15,56,195,157,229,17,1,212,79,65,246,2,237,7,185,129,214,8,64,1,7,239,98,52,89,
	],

	vk_ic: &[
		[
			28,4,47,228,225,176,129,88,169,48,149,136,93,37,42,52,29,47,200,185,228,230,201,195,63,150,101,94,239,27,188,32,
			22,39,93,63,176,199,25,227,110,216,46,151,119,85,22,14,238,126,65,192,182,104,217,56,88,252,68,210,37,187,88,64,
		],
		[
			24,87,142,24,232,179,254,61,177,194,186,45,96,157,120,251,179,106,76,38,213,11,6,179,18,27,22,170,119,18,27,191,
			46,2,250,132,123,252,94,100,88,128,247,133,121,247,187,224,229,66,223,37,9,55,176,253,152,190,148,31,252,141,118,46,
		],
	]
};