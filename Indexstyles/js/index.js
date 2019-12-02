$(document).ready(function() {
	/* Use Bootstrap table */
	$('table').addClass('table table-hover');
	/* Remove align="right" */
	$('td').removeAttr('align');
	/* Move table header in thead */
	var header = $('table tbody tr').first();
	$('table').prepend($('<thead/>').append(header));
	/* Add sorting arrows */
	var filter = '';
	var name = $('a', header).eq(0);
	var date = $('a', header).eq(1);
	var size = $('a', header).eq(2);
	if (name.attr('href') === '?C=N;O=A') {
		name.attr('href', '.');
	}
	if(/\/$/.test(document.URL) || /\?C=N;O=A$/.test(document.URL)) {
		name.append('<span class="icon icon-up">');
	} else if(/\?C=N;O=D$/.test(document.URL)) {
		filter = '?C=N;O=D';
		name.append('<span class="icon icon-down">');
	} else if(/\?C=M;O=A$/.test(document.URL)) {
		filter = '?C=M;O=A';
		date.append('<span class="icon icon-up">');
	} else if(/\?C=M;O=D$/.test(document.URL)) {
		filter = '?C=M;O=D';
		date.append('<span class="icon icon-down">');
	} else if(/\?C=S;O=A$/.test(document.URL)) {
		filter = '?C=S;O=A';
		size.append('<span class="icon icon-up">');
	} else if(/\?C=S;O=D$/.test(document.URL)) {
		filter = '?C=S;O=D';
		size.append('<span class="icon icon-down">');
	}
	/* Update home link */
	var home = $('a.navbar-brand');
	home.attr('href', home.attr('href') + filter);
	/* Parse each row */
	$('table tbody tr').each(function(i) {
		var a = $('a', this).first();
		var name = a.text();
		/* Remove "Parent Directory" row */
		if (name === 'Parent Directory' && i === 0) {
			if ($.trim($('td', this).last().html()) === '-') {
				$(this).remove();
			}
		/* Set directory row style */
		} else if (name.substring(name.length - 1) === '/') {
			$(this).addClass('active');
			$('td', this).last().html('');
			a.attr('href', a.attr('href') + filter);
			a.prepend('<span class="icon icon-folder">');
		/* Set file row style */
		} else {
			var suffix = name.substr(name.lastIndexOf('.') + 1).toLowerCase();
			if (suffix in suffixdb) {
				a.prepend('<span class="icon icon-file-' + suffixdb[suffix] + '">');
			} else {
				a.prepend('<span class="icon icon-file">');
			}
		}
	});
	/* Create Breadcrumb */
	var directories = document.URL.split('/').slice(3, -1);
	if (directories.length > 0) {
		var href = '/';
		$('.breadcrumb li').removeAttr('class').wrapInner('<a href="' + href + filter + '">');
		$.each(directories, function(i ,directory) {
			href += directory + '/';
			$('.breadcrumb').append('<li><a href="' + href + filter + '">' + decodeURIComponent(directory));
		});
		$('.breadcrumb li').last().addClass('active').children('a').contents().unwrap();
	}
	/* Show table */
	$('#breadcrumb').show();
	$('#table').show();
	/* Decode url */
	$('.url').each(function() {
		var url = decodeURIComponent($(this).text());
		$(this).html(/^[^?]*/.exec(url)[0]);
	});
	/* Back button */
	$('.btn').click(function() {
		history.go(-1);
	});
	
	/* Initialize carousels */
	$('#awesome-autoindex .owl-carousel').owlCarousel({
		lazyLoad: true,
		nav: true,
		navText: ['Before', 'After'],
		dots: false,
		items: 1
	});

	$('#multiple-templates .owl-carousel').owlCarousel({
		lazyLoad: true,
		loop: true,
		items: 1
	});

	$('#error-pages .owl-carousel').owlCarousel({
		lazyLoad: true,
		loop: true,
		items: 1
	});

	/* Copy to clipboard */
	$('#one-liner button').tooltip({
		container: 'body',
		placement: 'bottom',
		title: 'Copy to clipboard',
		trigger: 'hover'
	}).on('click', function() {
		var input = $('#one-liner input')[0];
		input.setSelectionRange(0, input.value.length + 1);
		try {
			var success = document.execCommand('copy');
			if (success) {
				$('#one-liner button').trigger('copied', ['Copied!']);
			} else {
				$('#one-liner button').trigger('copied', ['Copy with Ctrl-c']);
			}
		} catch (error) {
			$('#one-liner button').trigger('copied', ['Copy with Ctrl-c']);
		}
	}).on('copied', function(event, message) {
		$(this).attr('title', message)
			.tooltip('fixTitle')
			.tooltip('show')
			.attr('title', 'Copy to clipboard')
			.tooltip('fixTitle');
	});
});
