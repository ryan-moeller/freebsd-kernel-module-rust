OBJECTDIR?=target/objects

KMOD=	geom_md
SRCS=	md.c
OBJS=	$(OBJECTDIR)/*.o

.include<bsd.kmod.mk>
