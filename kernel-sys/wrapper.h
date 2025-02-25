// Copyright (c) 2022 NCC Group
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Based on public domain code by Johannes Lundberg

#include "opt_geom.h"
#include "opt_md.h"

#include <sys/param.h>  /* defines used in kernel.h */
#include <sys/systm.h>  /* uprintf */
#include <sys/bio.h>
#include <sys/conf.h>   /* cdevsw struct */
#include <sys/devicestat.h>
#include <sys/fcntl.h>
#include <sys/kernel.h> /* types used in module initialization */
#include <sys/kthread.h>
#include <sys/limits.h>
#include <sys/lock.h>
#include <sys/malloc.h>
#include <sys/mdioctl.h>
#include <sys/mount.h>
#include <sys/mutex.h>
#include <sys/namei.h>
//#include <sys/proc.h>
//#include <sys/sched.h>
#include <sys/sf_buf.h>
#include <sys/sysctl.h>
#include <sys/uio.h>    /* uio struct */
#include <sys/unistd.h>
#include <sys/vnode.h>
#include <sys/disk.h>

#include <geom/geom.h>
#include <geom/geom_int.h>

/*
#include <vm/vm.h>
#include <vm/vm_extern.h>
#include <vm/vm_param.h>
#include <vm/vm_object.h>
#include <vm/vm_page.h>
#include <vm/vm_pager.h>
#include <vm/swap_pager.h>
#include <vm/uma.h>
*/

#include <machine/bus.h>
