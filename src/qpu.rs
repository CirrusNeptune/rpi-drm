#![allow(nonstandard_style, dead_code)]

pub enum qpu_sig_bits {
    sig_brk = 0,
    sig_none = 1,
    sig_thread_switch = 2,
    sig_end = 3,
    sig_wait_score = 4,
    sig_unlock_score = 5,
    sig_last_thread_switch = 6,
    sig_coverage_load = 7,
    sig_color_load = 8,
    sig_color_load_end = 9,
    sig_load_tmu0 = 10,
    sig_load_tmu1 = 11,
    sig_alpha_mask_load = 12,
    sig_small_imm = 13,
    sig_load_imm = 14,
    sig_branch = 15,
}

pub enum qpu_waddr_a {
    ra0 = 0,
    ra1 = 1,
    ra2 = 2,
    ra3 = 3,
    ra4 = 4,
    ra5 = 5,
    ra6 = 6,
    ra7 = 7,
    ra8 = 8,
    ra9 = 9,
    ra10 = 10,
    ra11 = 11,
    ra12 = 12,
    ra13 = 13,
    ra14 = 14,
    ra15 = 15,
    ra16 = 16,
    ra17 = 17,
    ra18 = 18,
    ra19 = 19,
    ra20 = 20,
    ra21 = 21,
    ra22 = 22,
    ra23 = 23,
    ra24 = 24,
    ra25 = 25,
    ra26 = 26,
    ra27 = 27,
    ra28 = 28,
    ra29 = 29,
    ra30 = 30,
    ra31 = 31,
    r0 = 32,
    r1 = 33,
    r2 = 34,
    r3 = 35,
    tmu_noswap = 36,
    r5 = 37,
    host_int = 38,
    nop = 39,
    uniforms_addr = 40,
    quad_x = 41,
    ms_flags = 42,
    tlb_stencil_setup = 43,
    tlb_z = 44,
    tlb_color_ms = 45,
    tlb_color_all = 46,
    tlb_alpha_mask = 47,
    vpm = 48,
    vr_setup = 49,
    vr_addr = 50,
    mutex_release = 51,
    sfu_recip = 52,
    sfu_recipsqrt = 53,
    sfu_exp = 54,
    sfu_log = 55,
    tmu0_s = 56,
    tmu0_t = 57,
    tmu0_r = 58,
    tmu0_b = 59,
    tmu1_s = 60,
    tmu1_t = 61,
    tmu1_r = 62,
    tmu1_b = 63,
}

pub enum qpu_waddr_b {
    rb0 = 0,
    rb1 = 1,
    rb2 = 2,
    rb3 = 3,
    rb4 = 4,
    rb5 = 5,
    rb6 = 6,
    rb7 = 7,
    rb8 = 8,
    rb9 = 9,
    rb10 = 10,
    rb11 = 11,
    rb12 = 12,
    rb13 = 13,
    rb14 = 14,
    rb15 = 15,
    rb16 = 16,
    rb17 = 17,
    rb18 = 18,
    rb19 = 19,
    rb20 = 20,
    rb21 = 21,
    rb22 = 22,
    rb23 = 23,
    rb24 = 24,
    rb25 = 25,
    rb26 = 26,
    rb27 = 27,
    rb28 = 28,
    rb29 = 29,
    rb30 = 30,
    rb31 = 31,
    r0 = 32,
    r1 = 33,
    r2 = 34,
    r3 = 35,
    tmu_noswap = 36,
    r5 = 37,
    host_int = 38,
    nop = 39,
    uniforms_addr = 40,
    quad_y = 41,
    rev_flags = 42,
    tlb_stencil_setup = 43,
    tlb_z = 44,
    tlb_color_ms = 45,
    tlb_color_all = 46,
    tlb_alpha_mask = 47,
    vpm = 48,
    vw_setup = 49,
    vw_addr = 50,
    mutex_release = 51,
    sfu_recip = 52,
    sfu_recipsqrt = 53,
    sfu_exp = 54,
    sfu_log = 55,
    tmu0_s = 56,
    tmu0_t = 57,
    tmu0_r = 58,
    tmu0_b = 59,
    tmu1_s = 60,
    tmu1_t = 61,
    tmu1_r = 62,
    tmu1_b = 63,
}

pub enum qpu_mux {
    r0 = 0,
    r1 = 1,
    r2 = 2,
    r3 = 3,
    r4 = 4,
    r5 = 5,
    a = 6,
    b = 7,
}

pub enum qpu_raddr_a {
    ra0 = 0,
    ra1 = 1,
    ra2 = 2,
    ra3 = 3,
    ra4 = 4,
    ra5 = 5,
    ra6 = 6,
    ra7 = 7,
    ra8 = 8,
    ra9 = 9,
    ra10 = 10,
    ra11 = 11,
    ra12 = 12,
    ra13 = 13,
    ra14 = 14,
    pay_w = 15,
    ra16 = 16,
    ra17 = 17,
    ra18 = 18,
    ra19 = 19,
    ra20 = 20,
    ra21 = 21,
    ra22 = 22,
    ra23 = 23,
    ra24 = 24,
    ra25 = 25,
    ra26 = 26,
    ra27 = 27,
    ra28 = 28,
    ra29 = 29,
    ra30 = 30,
    ra31 = 31,
    uni = 32,
    vary = 35,
    elem = 38,
    nop = 39,
    x_pix = 41,
    ms_flags = 42,
    vpm_read = 48,
    vpm_ld_busy = 49,
    vpm_ld_wait = 50,
    mutex_acq = 51,
}

pub enum qpu_raddr_a_branch {
    ra0 = 0,
    ra1 = 1,
    ra2 = 2,
    ra3 = 3,
    ra4 = 4,
    ra5 = 5,
    ra6 = 6,
    ra7 = 7,
    ra8 = 8,
    ra9 = 9,
    ra10 = 10,
    ra11 = 11,
    ra12 = 12,
    ra13 = 13,
    ra14 = 14,
    pay_w = 15,
    ra16 = 16,
    ra17 = 17,
    ra18 = 18,
    ra19 = 19,
    ra20 = 20,
    ra21 = 21,
    ra22 = 22,
    ra23 = 23,
    ra24 = 24,
    ra25 = 25,
    ra26 = 26,
    ra27 = 27,
    ra28 = 28,
    ra29 = 29,
    ra30 = 30,
    ra31 = 31,
}

pub enum qpu_raddr_b {
    rb0 = 0,
    rb1 = 1,
    rb2 = 2,
    rb3 = 3,
    rb4 = 4,
    rb5 = 5,
    rb6 = 6,
    rb7 = 7,
    rb8 = 8,
    rb9 = 9,
    rb10 = 10,
    rb11 = 11,
    rb12 = 12,
    rb13 = 13,
    rb14 = 14,
    pay_z = 15,
    rb16 = 16,
    rb17 = 17,
    rb18 = 18,
    rb19 = 19,
    rb20 = 20,
    rb21 = 21,
    rb22 = 22,
    rb23 = 23,
    rb24 = 24,
    rb25 = 25,
    rb26 = 26,
    rb27 = 27,
    rb28 = 28,
    rb29 = 29,
    rb30 = 30,
    rb31 = 31,
    uni = 32,
    vary = 35,
    elem = 38,
    nop = 39,
    y_pix = 41,
    rev_flag = 42,
    vpm_read = 48,
    vpm_st_busy = 49,
    vpm_st_wait = 50,
    mutex_acq = 51,
}

pub enum qpu_op_add {
    nop = 0,
    fadd = 1,
    fsub = 2,
    fmin = 3,
    fmax = 4,
    fminabs = 5,
    fmaxabs = 6,
    ftoi = 7,
    itof = 8,
    add = 12,
    sub = 13,
    shr = 14,
    asr = 15,
    ror = 16,
    shl = 17,
    min = 18,
    max = 19,
    and = 20,
    or = 21,
    xor = 22,
    not = 23,
    clz = 24,
    v8adds = 30,
    v8subs = 31,
}

pub enum qpu_op_mul {
    nop = 0,
    fmul = 1,
    mul24 = 2,
    v8muld = 3,
    v8min = 4,
    v8max = 5,
    v8adds = 6,
    v8subs = 7,
}

pub enum qpu_pack_a {
    nop = 0,
    _16a = 1,
    _16b = 2,
    _8888 = 3,
    _8a = 4,
    _8b = 5,
    _8c = 6,
    _8d = 7,
    _32_sat = 8,
    _16a_sat = 9,
    _16b_sat = 10,
    _8888_sat = 11,
    _8a_sat = 12,
    _8b_sat = 13,
    _8c_sat = 14,
    _8d_sat = 15,
}

pub enum qpu_pack_mul {
    nop = 0,
    _8888 = 3,
    _8a = 4,
    _8b = 5,
    _8c = 6,
    _8d = 7,
}

pub enum qpu_unpack {
    nop = 0,
    _16a = 1,
    _16b = 2,
    _8d_rep = 3,
    _8a = 4,
    _8b = 5,
    _8c = 6,
    _8d = 7,
}

pub enum qpu_cond {
    never = 0,
    always = 1,
    zs = 2,
    zc = 3,
    ns = 4,
    nc = 5,
    cs = 6,
    cc = 7,
}

pub enum qpu_branch_cond {
    all_zs = 0,
    all_zc = 1,
    any_zs = 2,
    any_zc = 3,
    all_ns = 4,
    all_nc = 5,
    any_ns = 6,
    any_nc = 7,
    all_cs = 8,
    all_cc = 9,
    any_cs = 10,
    any_cc = 11,
    always = 15,
}

pub enum qpu_small_imm {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
    _9 = 9,
    _10 = 10,
    _11 = 11,
    _12 = 12,
    _13 = 13,
    _14 = 14,
    _15 = 15,
    _n16 = 16,
    _n15 = 17,
    _n14 = 18,
    _n13 = 19,
    _n12 = 20,
    _n11 = 21,
    _n10 = 22,
    _n9 = 23,
    _n8 = 24,
    _n7 = 25,
    _n6 = 26,
    _n5 = 27,
    _n4 = 28,
    _n3 = 29,
    _n2 = 30,
    _n1 = 31,
    _1_1 = 32,
    _2_1 = 33,
    _4_1 = 34,
    _8_1 = 35,
    _16_1 = 36,
    _32_1 = 37,
    _64_1 = 38,
    _128_1 = 39,
    _1_256 = 40,
    _1_128 = 41,
    _1_64 = 42,
    _1_32 = 43,
    _1_16 = 44,
    _1_8 = 45,
    _1_4 = 46,
    _1_2 = 47,
    rotate_by_r5 = 48,
    rotate_by_1 = 50,
    rotate_by_2 = 51,
    rotate_by_3 = 52,
    rotate_by_4 = 53,
    rotate_by_5 = 54,
    rotate_by_6 = 55,
    rotate_by_7 = 56,
    rotate_by_8 = 57,
    rotate_by_9 = 58,
    rotate_by_10 = 59,
    rotate_by_11 = 60,
    rotate_by_12 = 61,
    rotate_by_13 = 62,
    rotate_by_14 = 63,
    rotate_by_15 = 64,
}

#[macro_export]
macro_rules! qpu {
    // Encode load32
    (@encode({sig_load_imm;
    $waddr_add:tt = load32 ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:tt);
    $waddr_mul:tt = load32 ($mul_a:ident, $mul_b:ident);
    }) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    ($cond_mul:ident) $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@assemble {$($tail)*} -> [$($out,)*
            ((($crate::qpu::qpu_sig_bits::sig_load_imm as u64) << 60) |
            (0b000 << 57) |
            (($pm as u64) << 56) |
            (($pack as u64) << 52) |
            (($crate::qpu::qpu_cond::$cond_add as u64) << 49) |
            (($crate::qpu::qpu_cond::$cond_mul as u64) << 46) |
            (($sf as u64) << 45) |
            (($ws as u64) << 44) |
            (($waddr_add as u64) << 38) |
            (($waddr_mul as u64) << 32) |
            (($add_a as u64) & 0xffffffff)),
        ])
    };

    // Encode load16
    (@encode({sig_load_imm;
    $waddr_add:tt = load16 ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:tt);
    $waddr_mul:tt = load16 ($mul_a:ident, $mul_b:ident);
    }) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    ($cond_mul:ident) $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@assemble {$($tail)*} -> [$($out,)*
            ((($crate::qpu::qpu_sig_bits::sig_load_imm as u64) << 60) |
            (((!$signed) as u64) << 58) |
            (0b1 << 57) |
            (($pm as u64) << 56) |
            (($pack as u64) << 52) |
            (($crate::qpu::qpu_cond::$cond_add as u64) << 49) |
            (($crate::qpu::qpu_cond::$cond_mul as u64) << 46) |
            (($sf as u64) << 45) |
            (($ws as u64) << 44) |
            (($waddr_add as u64) << 38) |
            (($waddr_mul as u64) << 32) |
            ((($add_a as u64) & 0xffff) << 16) |
            (($add_b as u64) & 0xffff)),
        ])
    };

    // Encode sem_inc
    (@encode({sig_load_imm;
    $waddr_add:tt = sem_inc ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:tt);
    $waddr_mul:tt = sem_inc ($mul_a:ident, $mul_b:ident);
    }) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    ($cond_mul:ident) $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@assemble {$($tail)*} -> [$($out,)*
            ((($crate::qpu::qpu_sig_bits::sig_load_imm as u64) << 60) |
            (0b100 << 57) |
            (($pm as u64) << 56) |
            (($pack as u64) << 52) |
            (($crate::qpu::qpu_cond::$cond_add as u64) << 49) |
            (($crate::qpu::qpu_cond::$cond_mul as u64) << 46) |
            (($sf as u64) << 45) |
            (($ws as u64) << 44) |
            (($waddr_add as u64) << 38) |
            (($waddr_mul as u64) << 32) |
            ((($add_b as u64) & 0x7ffffff) << 5) |
            (1 << 4) |
            (($add_a as u64) & 0xf)),
        ])
    };

    // Encode sem_dec
    (@encode({sig_load_imm;
    $waddr_add:tt = sem_dec ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:tt);
    $waddr_mul:tt = sem_dec ($mul_a:ident, $mul_b:ident);
    }) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    ($cond_mul:ident) $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@assemble {$($tail)*} -> [$($out,)*
            ((($crate::qpu::qpu_sig_bits::sig_load_imm as u64) << 60) |
            (0b100 << 57) |
            (($pm as u64) << 56) |
            (($pack as u64) << 52) |
            (($crate::qpu::qpu_cond::$cond_add as u64) << 49) |
            (($crate::qpu::qpu_cond::$cond_mul as u64) << 46) |
            (($sf as u64) << 45) |
            (($ws as u64) << 44) |
            (($waddr_add as u64) << 38) |
            (($waddr_mul as u64) << 32) |
            ((($add_b as u64) & 0x7ffffff) << 5) |
            (0 << 4) |
            (($add_a as u64) & 0xf)),
        ])
    };

    // sig_load_imm catchall
    (@encode({sig_load_imm;
    $waddr_add:tt = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:tt);
    $waddr_mul:tt = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    }) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    ($cond_mul:ident) $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        "invalid instruction syntax for sig_load_imm"
    };

    // Encode branch
    (@encode({sig_branch;
    $waddr_add:tt = branch ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:tt);
    $waddr_mul:tt = branch ($mul_a:ident, $mul_b:ident);
    }) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    ($cond_mul:ident) $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@assemble {$($tail)*} -> [$($out,)*
            ((($crate::qpu::qpu_sig_bits::sig_branch as u64) << 60) |
            (($crate::qpu::qpu_branch_cond::$add_b as u64) << 52) |
            (($rel as u64) << 51) |
            (($reg as u64) << 50) |
            (($crate::qpu::qpu_raddr_a_branch::$raddr_a as u64) << 45) |
            (($ws as u64) << 44) |
            (($waddr_add as u64) << 38) |
            (($waddr_mul as u64) << 32) |
            (($add_a as u64) & 0xffffffff)),
        ])
    };

    // sig_branch catchall
    (@encode({sig_branch;
    $waddr_add:tt = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:tt);
    $waddr_mul:tt = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    }) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    ($cond_mul:ident) $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        "invalid instruction syntax for sig_branch"
    };

    // Encode ALU
    (@encode({$sig:ident;
    $waddr_add:tt = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:tt);
    $waddr_mul:tt = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    }) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    ($cond_mul:ident) $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@assemble {$($tail)*} -> [$($out,)*
            ((($crate::qpu::qpu_sig_bits::$sig as u64) << 60) |
            (($crate::qpu::qpu_unpack::$unpack as u64) << 57) |
            (($pm as u64) << 56) |
            (($pack as u64) << 52) |
            (($crate::qpu::qpu_cond::$cond_add as u64) << 49) |
            (($crate::qpu::qpu_cond::$cond_mul as u64) << 46) |
            (($sf as u64) << 45) |
            (($ws as u64) << 44) |
            (($waddr_add as u64) << 38) |
            (($waddr_mul as u64) << 32) |
            (($crate::qpu::qpu_op_mul::$op_mul as u64) << 29) |
            (($crate::qpu::qpu_op_add::$op_add as u64) << 24) |
            (($crate::qpu::qpu_raddr_a::$raddr_a as u64) << 18) |
            (($raddr_b as u64) << 12) |
            (($crate::qpu::qpu_mux::$add_a as u64) << 9) |
            (($crate::qpu::qpu_mux::$add_b as u64) << 6) |
            (($crate::qpu::qpu_mux::$mul_a as u64) << 3) |
            (($crate::qpu::qpu_mux::$mul_b as u64) << 0)),
        ])
    };

    (@select_ws({$sig:ident;
    $waddr_add:ident = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:tt);
    $waddr_mul:ident = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    }) -> ($pm:tt, false) $mods:tt $cond_mul:tt $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@encode({$sig;
            ($crate::qpu::qpu_waddr_a::$waddr_add) = $op_add ($add_a, $add_b, $raddr_a, $raddr_b);
            ($crate::qpu::qpu_waddr_b::$waddr_mul) = $op_mul ($mul_a, $mul_b);}) -> ($pm, false) $mods $cond_mul $pack
        {$($tail)*} -> [$($out,)*])
    };

    (@select_ws({$sig:ident;
    $waddr_add:ident = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:tt);
    $waddr_mul:ident = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    }) -> ($pm:tt, true) $mods:tt $cond_mul:tt $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@encode({$sig;
            ($crate::qpu::qpu_waddr_b::$waddr_add) = $op_add ($add_a, $add_b, $raddr_a, $raddr_b);
            ($crate::qpu::qpu_waddr_a::$waddr_mul) = $op_mul ($mul_a, $mul_b);}) -> ($pm, true) $mods $cond_mul $pack
        {$($tail)*} -> [$($out,)*])
    };

    (@select_raddr_b({sig_small_imm;
    $waddr_add:ident = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:ident);
    $waddr_mul:ident = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    }) -> $pack_mods:tt $mods:tt $cond_mul:tt $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@select_ws({sig_small_imm;
            $waddr_add = $op_add ($add_a, $add_b, $raddr_a, ($crate::qpu::qpu_small_imm::$raddr_b));
            $waddr_mul = $op_mul ($mul_a, $mul_b);}) -> $pack_mods $mods $cond_mul $pack
        {$($tail)*} -> [$($out,)*])
    };

    (@select_raddr_b({$sig:ident;
    $waddr_add:ident = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:ident);
    $waddr_mul:ident = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    }) -> $pack_mods:tt $mods:tt $cond_mul:tt $pack:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@select_ws({$sig;
            $waddr_add = $op_add ($add_a, $add_b, $raddr_a, ($crate::qpu::qpu_raddr_b::$raddr_b));
            $waddr_mul = $op_mul ($mul_a, $mul_b);}) -> $pack_mods $mods $cond_mul $pack
        {$($tail)*} -> [$($out,)*])
    };

    (@parse_pack({$sig:ident;
    $waddr_add:ident .$add_pack:ident = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:ident);
    $waddr_mul:ident = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    }) -> (false, false) $mods:tt $cond_mul:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@select_raddr_b({$sig;
            $waddr_add = $op_add ($add_a, $add_b, $raddr_a, $raddr_b);
            $waddr_mul = $op_mul ($mul_a, $mul_b);}) -> (false, false) $mods $cond_mul ($crate::qpu::qpu_pack_a::$add_pack)
        {$($tail)*} -> [$($out,)*])
    };

    (@parse_pack({$sig:ident;
    $waddr_add:ident = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:ident);
    $waddr_mul:ident .$mul_pack:ident = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    }) -> (true, $ws:tt) $mods:tt $cond_mul:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@select_raddr_b({$sig;
            $waddr_add = $op_add ($add_a, $add_b, $raddr_a, $raddr_b);
            $waddr_mul = $op_mul ($mul_a, $mul_b);}) -> (true, $ws) $mods $cond_mul ($crate::qpu::qpu_pack_mul::$mul_pack)
        {$($tail)*} -> [$($out,)*])
    };

    (@parse_pack({$sig:ident;
    $waddr_add:ident = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:ident);
    $waddr_mul:ident = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    }) -> $pack_mods:tt $mods:tt $cond_mul:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@select_raddr_b({$sig;
            $waddr_add = $op_add ($add_a, $add_b, $raddr_a, $raddr_b);
            $waddr_mul = $op_mul ($mul_a, $mul_b);}) -> $pack_mods $mods $cond_mul ($crate::qpu::qpu_pack_a::nop)
        {$($tail)*} -> [$($out,)*])
    };

    (@parse_mul_cond($inst:tt, .never) -> $pack_mods:tt $mods:tt ($cond_mul:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mul_cond($inst,) -> $pack_mods $mods (never)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mul_cond($inst:tt, .always) -> $pack_mods:tt $mods:tt ($cond_mul:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mul_cond($inst,) -> $pack_mods $mods (always)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mul_cond($inst:tt, .zs) -> $pack_mods:tt $mods:tt ($cond_mul:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mul_cond($inst,) -> $pack_mods $mods (zs)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mul_cond($inst:tt, .zc) -> $pack_mods:tt $mods:tt ($cond_mul:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mul_cond($inst,) -> $pack_mods $mods (zc)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mul_cond($inst:tt, .ns) -> $pack_mods:tt $mods:tt ($cond_mul:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mul_cond($inst,) -> $pack_mods $mods (ns)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mul_cond($inst:tt, .nc) -> $pack_mods:tt $mods:tt ($cond_mul:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mul_cond($inst,) -> $pack_mods $mods (nc)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mul_cond($inst:tt, .cs) -> $pack_mods:tt $mods:tt ($cond_mul:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mul_cond($inst,) -> $pack_mods $mods (cs)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mul_cond($inst:tt, .cc) -> $pack_mods:tt $mods:tt ($cond_mul:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mul_cond($inst,) -> $pack_mods $mods (cc)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mul_cond({$sig:ident;
    $waddr_add:ident $(.$add_pack:ident)? = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:ident);
    $waddr_mul:ident $(.$mul_pack:ident)? = $op_mul:ident ($mul_a:ident, $mul_b:ident);
    },) -> $pack_mods:tt $mods:tt $cond_mul:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_pack({$sig;
            $waddr_add $(.$add_pack)? = $op_add ($add_a, $add_b, $raddr_a, $raddr_b);
            $waddr_mul $(.$mul_pack)? = $op_mul ($mul_a, $mul_b);}) -> $pack_mods $mods $cond_mul
        {$($tail)*} -> [$($out,)*])
    };

    (@parse_mods($inst:tt, .pm $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> (true, $ws) ($sf, $rel, $reg, $signed, $cond_add, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .ws $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, true) ($sf, $rel, $reg, $signed, $cond_add, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .sf $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) (true, $rel, $reg, $signed, $cond_add, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .rel $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, true, $reg, $signed, $cond_add, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .reg $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, true, $signed, $cond_add, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .signed $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, true, $cond_add, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .never $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, never, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .always $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, always, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .zs $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, zs, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .zc $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, zc, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .ns $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, ns, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .nc $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, nc, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .cs $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, cs, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .cc $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, cc, $unpack)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, .nop $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, $cond_add, nop)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, ._16a $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, $cond_add, _16a)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, ._16b $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, $cond_add, _16b)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, ._8d_rep $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, $cond_add, _8d_rep)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, ._8a $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, $cond_add, _8a)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, ._8b $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, $cond_add, _8b)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, ._8c $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, $cond_add, _8c)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods($inst:tt, ._8d $(.$mods:ident)*) -> ($pm:tt, $ws:tt) ($sf:literal, $rel:literal, $reg:literal, $signed:literal, $cond_add:ident, $unpack:ident)
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods($inst, $(.$mods)*) -> ($pm, $ws) ($sf, $rel, $reg, $signed, $cond_add, _8d)
        {$($tail)*} -> [$($out,)*])
    };
    (@parse_mods({$sig:ident;
    $waddr_add:ident $(.$add_pack:ident)? = $op_add:ident ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:ident);
    $waddr_mul:ident $(.$mul_pack:ident)? = $op_mul:ident $(.$cond_mul:ident)? ($mul_a:ident, $mul_b:ident);
    },) -> $pack_mods:tt $mods:tt
    {$($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mul_cond({$sig;
            $waddr_add $(.$add_pack)? = $op_add ($add_a, $add_b, $raddr_a, $raddr_b);
            $waddr_mul $(.$mul_pack)? = $op_mul ($mul_a, $mul_b);}, $(.$cond_mul)?) -> $pack_mods $mods (never)
        {$($tail)*} -> [$($out,)*])
    };

    (@assemble {sig_load_imm;
    $waddr_add:ident $(.$add_pack:ident)? = load32 $(.$mods:ident)* ($add_a:expr);
    $waddr_mul:ident $(.$mul_pack:ident)? = load32 $(.$cond_mul:ident)? ();
    $($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods({sig_load_imm;
            $waddr_add $(.$add_pack)? = load32 ($add_a, nop, nop, nop);
            $waddr_mul $(.$mul_pack)? = load32 $(.$cond_mul)? (nop, nop);}, $(.$mods)*) -> (false, false) (false, false, false, false, never, nop)
        {$($tail)*} -> [$($out,)*])
    };

    (@assemble {sig_load_imm;
    $waddr_add:ident $(.$add_pack:ident)? = $op_add:ident $(.$mods:ident)* ($add_a:expr, $add_b:expr);
    $waddr_mul:ident $(.$mul_pack:ident)? = $op_mul:ident $(.$cond_mul:ident)? ();
    $($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods({sig_load_imm;
            $waddr_add $(.$add_pack)? = $op_add ($add_a, $add_b, nop, nop);
            $waddr_mul $(.$mul_pack)? = $op_mul $(.$cond_mul)? (nop, nop);}, $(.$mods)*) -> (false, false) (false, false, false, false, never, nop)
        {$($tail)*} -> [$($out,)*])
    };

    (@assemble {sig_load_imm;
    $waddr_add:ident $(.$add_pack:ident)? = $op_add:ident $(.$mods:ident)* ($add_a:expr $(, $add_b:expr $(, $raddr_a:ident, $raddr_b:ident)?)?);
    $waddr_mul:ident $(.$mul_pack:ident)? = $op_mul:ident $(.$cond_mul:ident)? ($($mul_a:ident, $mul_b:ident)?);
    $($tail:tt)*} -> [$($out:tt,)*]) => {
        "bad op parameter syntax for sig_load_imm"
    };

    (@assemble {sig_branch;
    $waddr_add:ident = branch $(.$mods:ident)* ($add_a:expr, $add_b:tt, $raddr_a:ident);
    $waddr_mul:ident = branch ();
    $($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods({sig_branch;
            $waddr_add = branch ($add_a, $add_b, $raddr_a, nop);
            $waddr_mul = branch (nop, nop);}, $(.$mods)*) -> (false, false) (false, false, false, false, never, nop)
        {$($tail)*} -> [$($out,)*])
    };

    (@assemble {sig_branch;
    $waddr_add:ident = branch $(.$mods:ident)* ($add_a:expr, $add_b:tt);
    $waddr_mul:ident = branch ();
    $($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods({sig_branch;
            $waddr_add = branch ($add_a, $add_b, ra0, nop);
            $waddr_mul = branch (nop, nop);}, $(.$mods)*) -> (false, false) (false, false, false, false, never, nop)
        {$($tail)*} -> [$($out,)*])
    };

    (@assemble {sig_branch;
    $waddr_add:ident $(.$add_pack:ident)? = $op_add:ident $(.$mods:ident)* ($add_a:expr $(, $add_b:tt $(, $raddr_a:ident, $raddr_b:ident)?)?);
    $waddr_mul:ident $(.$mul_pack:ident)? = $op_mul:ident $(.$cond_mul:ident)? ($($mul_a:ident, $mul_b:ident)?);
    $($tail:tt)*} -> [$($out:tt,)*]) => {
        "bad op parameter syntax for sig_branch"
    };

    (@assemble {$sig:ident;
    $waddr_add:ident $(.$add_pack:ident)? = $op_add:ident $(.$mods:ident)* ($add_a:tt, $add_b:tt);
    $waddr_mul:ident $(.$mul_pack:ident)? = $op_mul:ident $(.$cond_mul:ident)? ($mul_a:ident, $mul_b:ident);
    $($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@assemble {$sig;
            $waddr_add $(.$add_pack)? = $op_add $(.$mods)* ($add_a, $add_b, nop, nop);
            $waddr_mul $(.$mul_pack)? = $op_mul $(.$cond_mul)? ($mul_a, $mul_b);
            $($tail)*} -> [$($out,)*])
    };

    (@assemble {$sig:ident;
    $waddr_add:ident $(.$add_pack:ident)? = $op_add:ident $(.$mods:ident)* ($add_a:tt, $add_b:tt, $raddr_a:ident, $raddr_b:ident);
    $waddr_mul:ident $(.$mul_pack:ident)? = $op_mul:ident $(.$cond_mul:ident)? ($mul_a:ident, $mul_b:ident);
    $($tail:tt)*} -> [$($out:tt,)*]) => {
        qpu!(@parse_mods({$sig;
            $waddr_add $(.$add_pack)? = $op_add ($add_a, $add_b, $raddr_a, $raddr_b);
            $waddr_mul $(.$mul_pack)? = $op_mul $(.$cond_mul)? ($mul_a, $mul_b);}, $(.$mods)*) -> (false, false) (false, false, false, false, never, nop)
        {$($tail)*} -> [$($out,)*])
    };

    (@assemble {} -> [$($out:tt,)*]) => {
        [$($out,)*]
    };

    ($sig:ident;
    $waddr_add:ident $(.$add_pack:ident)? = $op_add:ident $(.$mods:ident)* ($add_a:tt $(, $add_b:tt $(, $raddr_a:ident $(, $raddr_b:ident)?)?)?);
    $waddr_mul:ident $(.$mul_pack:ident)? = $op_mul:ident $(.$cond_mul:ident)? ($($mul_a:ident, $mul_b:ident)?);
    $($tail:tt)*) => {
        qpu!(@assemble {
            $sig;
            $waddr_add $(.$add_pack)? = $op_add $(.$mods)* ($add_a $(, $add_b $(, $raddr_a $(, $raddr_b)?)?)?);
            $waddr_mul $(.$mul_pack)? = $op_mul $(.$cond_mul)? ($($mul_a, $mul_b)?);
            $($tail)*
        } -> [])
    };

    ($sig:ident;
    $waddr_add:ident $(.$add_pack:ident)? = $op_add:ident $(.$mods:ident)* ($add_a:expr $(, $add_b:expr $(, $raddr_a:ident $(, $raddr_b:ident)?)?)?);
    $waddr_mul:ident $(.$mul_pack:ident)? = $op_mul:ident $(.$cond_mul:ident)? ($($mul_a:ident, $mul_b:ident)?);
    $($tail:tt)*) => {
        qpu!(@assemble {
            $sig;
            $waddr_add $(.$add_pack)? = $op_add $(.$mods)* ($add_a $(, $add_b $(, $raddr_a $(, $raddr_b)?)?)?);
            $waddr_mul $(.$mul_pack)? = $op_mul $(.$cond_mul)? ($($mul_a, $mul_b)?);
            $($tail)*
        } -> [])
    };
}

#[repr(u8)]
enum VpmSize {
    _8bit = 0,
    _16bit = 1,
    _32bit = 2,
}

pub const fn vpm_block_read_horizontal_32(num_reads: u8, stride: u8, y: u8) -> u32 {
    ((num_reads as u32) << 20)
        | ((stride as u32) << 12)
        | (1 << 11)
        | ((VpmSize::_32bit as u32) << 8)
        | (y as u32)
}

pub const fn vpm_block_write_horizontal_32(stride: u8, y: u8) -> u32 {
    ((stride as u32) << 12) | (1 << 11) | ((VpmSize::_32bit as u32) << 8) | (y as u32)
}

pub const fn vpm_block_read_vertical_32(num_reads: u8, stride: u8, y: u8, x: u8) -> u32 {
    ((num_reads as u32) << 20)
        | ((stride as u32) << 12)
        | (0 << 11)
        | ((VpmSize::_32bit as u32) << 8)
        | ((y as u32) << 4)
        | (x as u32)
}

pub const fn vpm_block_write_vertical_32(stride: u8, y: u8, x: u8) -> u32 {
    ((stride as u32) << 12)
        | (0 << 11)
        | ((VpmSize::_32bit as u32) << 8)
        | ((y as u32) << 4)
        | (x as u32)
}
