VERSION  ?= latest
DOCKER   ?= docker
IMG_NAME ?= rung_cam

build_rung: .rung.docker_built

.rung.docker_built: Dockerfile
	$(DOCKER) run --privileged --rm tonistiigi/binfmt --install all
	$(DOCKER) run --rm --privileged multiarch/qemu-user-static --reset -p yes -c yes
	$(DOCKER) build -t $(IMG_NAME):$(VERSION) .
	touch .rung.docker_built

run_rung: .rung.docker_built
	$(DOCKER) run -v $(abspath .):/build -it $(IMG_NAME):$(VERSION) /bin/bash

build.rung: .rung.docker_built
	$(DOCKER) run -v $(abspath .):/build -it $(IMG_NAME):$(VERSION) cargo-xbuild

build.rung.noit: .rung.docker_built
	$(DOCKER) run -v $(abspath .):/build $(IMG_NAME):$(VERSION) cargo-xbuild

clean:
	rm -f .rung.docker_built